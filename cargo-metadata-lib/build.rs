use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use cargo_metadata::MetadataCommand;

// CrateInfo from lib.rs, assuming it's pub and defined as such:
// #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
// pub struct CrateInfo {
//     pub name: &'static str,
//     pub path: &'static str,
// }
// Since build.rs cannot directly import from lib.rs of the same crate,
// we'll define a temporary CrateInfo struct to hold String values
// and convert to &'static str using .leak() at the end.
#[derive(Debug)]
struct TempCrateInfo {
    name: String,
    path: String,
}


fn main() -> Result<()> {
    // Monitor changes to relevant Cargo.toml files that might affect metadata
    println!("cargo:rerun-if-changed=Cargo.toml"); // This crate's Cargo.toml
    println!("cargo:rerun-if-changed=../../Cargo.toml"); // Top-level workspace Cargo.toml

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let dest_path = out_dir.join("cargo_tree_data.rs");

    let metadata = MetadataCommand::new()
        .exec()
        .context("Failed to execute `cargo metadata` command")?;

    let mut crate_data_temp = Vec::new();
    
    // CARGO_MANIFEST_DIR for cargo-metadata-lib/build.rs is the cargo-metadata-lib directory
    let cargo_metadata_lib_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    
    // The current project root is one level up from cargo-metadata-lib, i.e., split-decls-rs
    // This is the base for calculating relative paths.
    let current_project_root = cargo_metadata_lib_dir.parent() 
        .context("Could not get parent directory of cargo-metadata-lib")?.to_path_buf();


    for package in metadata.packages {
        // Only consider packages that are part of this workspace and have a local path
        if package.source.is_none() || package.source.unwrap().repr.starts_with("file://") {
            let package_path = PathBuf::from(package.manifest_path.parent().context("Package manifest path has no parent")?);
            
            let relative_path = pathdiff::diff_paths(&package_path, &current_project_root)
                .unwrap_or_else(|| package_path.clone());
            
            crate_data_temp.push(TempCrateInfo {
                name: package.name,
                path: relative_path.to_string_lossy().into_owned(),
            });
        }
    }

    let mut data_entries = Vec::new();
    for info in &crate_data_temp {
        // Convert TempCrateInfo to CrateInfo with &'static str using leak()
        data_entries.push(format!(
            r#"CrateInfo {{ name: \"{}\" , path: \"{}\" }}"#, 
            info.name.leak(), // Leak String to &'static str
            info.path.leak(), // Leak String to &'static str
        ));
    }

    let serialized_data = format!(
        "pub const CARGO_TREE_DATA: &[CrateInfo] = &[\n    {}\n]";
        data_entries.join(",\n    ")
    );

    fs::write(&dest_path, serialized_data)?;

    Ok(())
}
