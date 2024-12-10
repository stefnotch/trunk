use anyhow::Result;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> Result<ExitCode> {
    trunk::run_cli().await
}
