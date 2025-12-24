use clap::Parser;
use anyhow::{Context, Result};
use std::collections::{HashSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use toml::{self, Table, Value};
use serde::{Deserialize, Serialize};
use cargo_metadata_lib::get_cargo_tree_data;
use cargo_metadata_lib::CrateInfo;


// --- Structs for SplitDeclsConfig (from split-decls-types/src/lib.rs) ---
// Duplicated here to avoid circular dependency for this tool.
// In a real project, this would be a shared library.
#[derive(Debug, Default, Serialize, Deserialize)]
struct SplitDeclsConfig {
    #[serde(default)]
    wrapping: WrappingConfig,
    #[serde(default)]
    crate_path_overrides: HashMap<String, String>, // Changed from PathBuf for simpler TOML serialization
    pub explicit_crate_path_mappings: Option<HashMap<String, String>>, // Corrected to Option
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct WrappingConfig {
    #[serde(default)]
    crates: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct CratePathsOnly {
    #[serde(default)]
    crate_path_overrides: HashMap<String, String>,
}


fn main() -> Result<()> {
    let args = Args::parse();

    // 1. Load existing split-decls-rs.toml
    let mut split_decls_config_content = fs::read_to_string(&args.split_decls_config_path).unwrap_or_default();
    let mut split_decls_data: SplitDeclsConfig = toml::from_str(&split_decls_config_content).unwrap_or_default();

    let mut existing_crates_to_wrap: HashSet<String> = split_decls_data.wrapping.crates.drain(..).collect();
    let mut existing_path_overrides: HashMap<String, String> = split_decls_data.crate_path_overrides.drain().collect();

    // Use cargo-tree-macro to get dependency information
    for crate_info in get_cargo_tree_data() {
        existing_crates_to_wrap.insert(crate_info.name.to_string());
        existing_path_overrides.insert(crate_info.name.to_string(), crate_info.path.to_string());
    }

    // Special handling for the current project itself
    existing_crates_to_wrap.insert("split-decls-rs".to_string());
    existing_path_overrides.insert("split-decls-rs".to_string(), ".".to_string());
    
    // Filter out internal crates of split-decls-rs itself (if any from cargo tree)
    // and other crates that are part of the larger original workspace where RootCargo.toml is located
    // but are not intended to be wrapped by split-decls-rs
    let internal_crates_to_skip: HashSet<&str> = [
        "split-decls-types",
        "cargo-toml-generator-types",
        "cargo-toml-generator-macros",
        "cargo-toml-parts",
        "pagerank_rs",
        // Add other crate names here that should not be wrapped as individual units
        // or whose paths are handled explicitly.
    ].iter().cloned().collect();

    existing_crates_to_wrap.retain(|crate_name| {
        !internal_crates_to_skip.contains(crate_name.as_str()) && !crate_name.starts_with("split-decls-rs-")
    });


    // Sort crates for consistent output
    let mut sorted_crates: Vec<String> = existing_crates_to_wrap.into_iter().collect();
    sorted_crates.sort();
    split_decls_data.wrapping.crates = sorted_crates;
    
    // Apply explicit_crate_path_mappings from config, giving them precedence over derived paths
    if let Some(explicit_mappings) = split_decls_data.explicit_crate_path_mappings.take() { // Use .take() to move out of the Option
        for (crate_name, path_str) in explicit_mappings {
            existing_path_overrides.insert(crate_name, path_str);
        }
    }

    // Sort overrides by key for consistent output
    let mut sorted_overrides: Vec<(String, String)> = existing_path_overrides.into_iter().collect();
    sorted_overrides.sort_by(|a, b| a.0.cmp(&b.0));
    split_decls_data.crate_path_overrides = sorted_overrides.into_iter().collect();


    // Write updated split-decls-rs.toml
    let updated_toml_content = toml::to_string_pretty(&split_decls_data)
        .context("Failed to serialize updated split-decls-rs.toml")?;
    fs::write(&args.split_decls_config_path, updated_toml_content)
        .context(format!("Failed to write to {}", args.split_decls_config_path.display()))?;

    println!("Successfully updated {}", args.split_decls_config_path.display());

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the split-decls-rs.toml configuration file to update
    #[clap(long)]
    split_decls_config_path: PathBuf,
}

