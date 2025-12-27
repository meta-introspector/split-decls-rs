use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use crate::paths::CratePaths; 
use crate::add_generated_header;
use split_decls_types::SplitDeclsConfig;
use crate::patch_config;
use lib_cargo::CargoProcessor;

/// Generates the new Cargo.toml for the crate using lib-cargo processor
pub fn generate_new_cargotoml(
    original_cargo_toml_path: &Path,
    output_cargo_toml_path: &Path,
    original_crate_root_path: &Path,
    global_config: &SplitDeclsConfig,
    patch_config: &patch_config::PatchConfig,
    dry_run: bool,
) -> Result<()> {
    // Use the config directly - no conversion needed
    let processor = CargoProcessor::new(global_config.clone());
    processor.process_cargo_toml(
        original_cargo_toml_path,
        output_cargo_toml_path,
        original_crate_root_path,
        dry_run,
    )?;

    Ok(())
}