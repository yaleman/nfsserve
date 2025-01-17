use std::path::PathBuf;

use clap::Parser;
use nfsserve::demofs::DemoFS;
use nfsserve::mirrorfs::MirrorFS;
use nfsserve::tcp::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Config {
    #[arg(short, long)]
    port: Option<u32>,

    #[arg(short, long)]
    ip_address: Option<String>,
    #[arg(short = 'P', long)]
    path: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: Some(11111),
            ip_address: Some("127.0.0.1".to_string()),
            path: None,
        }
    }
}

impl Config {
    fn listener_address(&self) -> String {
        // either they specify one, or they get the default!
        let listener_port = &self.port.unwrap_or(Config::default().port.unwrap());
        let listener_ip = self
            .ip_address
            .clone()
            .unwrap_or(Config::default().ip_address.unwrap());

        format!("{}:{}", listener_ip, listener_port)
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_writer(std::io::stderr)
        .init();

    let config = Config::parse();
    let listener_address = config.listener_address();
    match config.path {
        Some(path) => {
            let listener = NFSTcpListener::bind(&listener_address, MirrorFS::new(path))
                .await
                .unwrap();
            listener.handle_forever().await.unwrap();
        }
        None => {
            let listener = NFSTcpListener::bind(&listener_address, DemoFS::default())
                .await
                .unwrap();
            listener.handle_forever().await.unwrap()
        }
    };
}
// Test with
// mount -t nfs -o nolocks,vers=3,tcp,port=12000,mountport=12000,soft 127.0.0.1:/ mnt/
