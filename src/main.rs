// for parsing arguments -> "cargo add clap --features derive" to add to the project
use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    // get content + take care of error
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    let mut i: u32 = 0;
    // search for line with pattern
    for line in content.lines() {
        i += 1;
        if line.contains(&args.pattern) {
            println!("line {num}: {line}", num = i, line = line);
        }
    }
    Ok(())
}
