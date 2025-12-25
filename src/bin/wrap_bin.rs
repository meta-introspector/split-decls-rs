use anyhow::Result;
use clap::Parser;
use split_decls_rs::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "wrap-bin")]
#[command(about = "Wrap a single crate and generate it in output2")]
struct Args {
    /// Path to the crate to wrap
    crate_path: PathBuf,
    
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Load config
    let config = load_split_decls_config("split-decls-rs.toml")?;
    
    // Generate wrapped crate
    let output_base = PathBuf::from("output2");
    
    if args.verbose {
        println!("Wrapping crate: {:?}", args.crate_path);
        println!("Output directory: {:?}", output_base);
    }
    
    // Use existing single crate generation logic
    generate_wrapped_crate(&args.crate_path, &output_base, &config)?;
    
    println!("✅ Successfully wrapped crate to output2");
    Ok(())
}
