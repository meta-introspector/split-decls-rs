use clap::Parser;
use anyhow::{Result, Context};
use std::path::PathBuf;
use std::collections::HashMap;

use split_decls_rs::{
    generate_wrapped_crate::generate_wrapped_crate,
    patch_config::PatchConfig,
};
use split_decls_types::SplitDeclsConfig;

#[derive(Parser, Debug)]
#[clap(author, version, about = "Wrap a single crate with minimal configuration")]
struct Args {
    /// Path to the crate directory (containing Cargo.toml)
    crate_path: PathBuf,

    /// Output directory (optional, defaults to ./output2)
    #[clap(short, long)]
    output: Option<PathBuf>,

    /// Verbose output
    #[clap(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    let output_dir = args.output.unwrap_or_else(|| PathBuf::from("output2"));
    
    // Extract crate name from Cargo.toml
    let cargo_toml_path = args.crate_path.join("Cargo.toml");
    let cargo_content = std::fs::read_to_string(&cargo_toml_path)
        .context("Failed to read Cargo.toml")?;
    let cargo_toml: toml::Value = toml::from_str(&cargo_content)
        .context("Failed to parse Cargo.toml")?;
    
    let crate_name = cargo_toml["package"]["name"]
        .as_str()
        .context("Failed to extract crate name from Cargo.toml")?
        .to_string();
    
    if args.verbose {
        println!("Processing crate: {} at {}", crate_name, args.crate_path.display());
        println!("Output directory: {}", output_dir.display());
    }
    
    // Create minimal configs
    let global_config = SplitDeclsConfig::default();
    
    let patch_config = PatchConfig::default();
    
    // Create output directory
    std::fs::create_dir_all(&output_dir)
        .context("Failed to create output directory")?;
    
    // Process the crate
    generate_wrapped_crate(
        &output_dir,
        &crate_name,
        &args.crate_path,
        &global_config,
        &patch_config,
        false, // dry_run
    ).context(format!("Failed to wrap crate '{}'", crate_name))?;

    println!("✅ Successfully wrapped crate '{}' to '{}'", crate_name, output_dir.display());

    Ok(())
}
