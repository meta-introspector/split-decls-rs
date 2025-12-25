use clap::Parser;
use anyhow::{Result, Context};
use std::path::{Path, PathBuf};

use split_decls_rs::{
    generate_wrapped_crate::generate_wrapped_crate,
    patch_config::PatchConfig,
};
use split_decls_types::SplitDeclsConfig;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the crate to wrap
    #[clap(long)]
    crate_name: String,

    /// Path to the original crate's root directory (containing its Cargo.toml)
    #[clap(long)]
    original_crate_path: PathBuf,

    /// Output directory for the wrapped crate
    #[clap(long)]
    output_dir: PathBuf,

    /// Path to the split-decls-rs.toml configuration file
    #[clap(long)]
    config_path: PathBuf,

    /// Path to the patch_config.toml configuration file
    #[clap(long)]
    patch_config_path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let global_config = SplitDeclsConfig::load_from_file(&args.config_path)
        .context(format!("Failed to load configuration from {}", args.config_path.display()))?;
    
    let patch_config = PatchConfig::load_from_file(&args.patch_config_path)
        .context(format!("Failed to load patch configuration from {}", args.patch_config_path.display()))?;

    generate_wrapped_crate(
        &args.output_dir,
        &args.crate_name,
        &args.original_crate_path,
        &global_config,
        &patch_config,
        false, // dry_run
    ).context(format!("Failed to wrap crate '{}'", args.crate_name))?;

    println!("Successfully wrapped crate '{}' to '{}'", args.crate_name, args.output_dir.display());

    Ok(())
}
