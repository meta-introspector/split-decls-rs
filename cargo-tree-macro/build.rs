use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};
use cargo_metadata::MetadataCommand;

#[derive(Debug, Serialize, Deserialize)]
struct CrateInfo {
    name: String,
    path: String,
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=../../Cargo.toml"); // Monitor changes to the overall workspace Cargo.toml
    println!("cargo:rerun-if-changed=../Cargo.toml");    // Monitor changes to split-decls-rs's Cargo.toml

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let dest_path = out_dir.join("cargo_tree_data.rs");

    // Execute cargo metadata to get structured information about the workspace
    let metadata = MetadataCommand::new()
        .exec()
        .context("Failed to execute `cargo metadata` command")?;

    let mut crate_data = Vec::new();
    let current_project_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?); // This is split-decls-rs

    // Iterate through all packages in the workspace
    for package in metadata.packages {
        // Only consider packages that are part of this workspace and have a local path
        if package.source.is_none() || package.source.unwrap().starts_with("file://") {
            let package_path = PathBuf::from(package.manifest_path.parent().context("Package manifest path has no parent")?);
            
            // Calculate path relative to the split-decls-rs crate root
            let relative_path = pathdiff::diff_paths(&package_path, &current_project_root)
                .unwrap_or_else(|| package_path.clone()); // Fallback if diff fails
            
            crate_data.push(CrateInfo {
                name: package.name,
                path: relative_path.to_string_lossy().into_owned(),
            });
        }
    }

    // Generate the Rust code that defines the CARGO_TREE_DATA constant
    let serialized_data = format!(
        "pub const CARGO_TREE_DATA: &[CrateInfo] = &{};",
        serde_json::to_string(&crate_data)? // Serialize to JSON array
    );

    fs::write(&dest_path, serialized_data)?;

    Ok(())
}