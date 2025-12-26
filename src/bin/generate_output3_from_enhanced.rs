use std::fs;
use anyhow::Result;
use split_decls_types::SplitDeclsConfig;
use split_decls_rs::*;
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    println!("Running full bootstrap using split-decls-rs functions");
    
    let config = SplitDeclsConfig::default();
    let patch_config = patch_config::PatchConfig::default();
    
    // Use the correct function signature
    let output_dir = PathBuf::from("output3");
    let module_not_found_errors = generate_wrapped_workspace(
        &output_dir,
        &patch_config,
        &config,
        Path::new("."),
        false, // dry_run
        true,  // verbose
        false  // cargo_only
    )?;
    
    println!("Bootstrap completed with {} errors!", module_not_found_errors.len());
    
    // Check what was generated
    if output_dir.exists() {
        let entries = fs::read_dir(&output_dir)?;
        let count = entries.count();
        println!("Generated {} items in output3", count);
    }
    
    Ok(())
}
