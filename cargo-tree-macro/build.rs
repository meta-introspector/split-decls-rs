use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{Context, Result};
use cargo_metadata::MetadataCommand;

include!("src/common_data.rs"); // Include the shared data definition directly
// Now CrateInfo is directly in scope


fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=../../Cargo.toml"); // Monitor changes to the overall workspace Cargo.toml
    println!("cargo:rerun-if-changed=../Cargo.toml");    // Monitor changes to split-decls-rs's Cargo.toml

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let dest_path = out_dir.join("cargo_tree_data.rs");

    let metadata = MetadataCommand::new()
        .exec()
        .context("Failed to execute `cargo metadata` command")?;

    let mut crate_data = Vec::new();
    let current_project_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);

    for package in metadata.packages {
        if package.source.is_none() || package.source.unwrap().repr.starts_with("file://") {
            let package_path = PathBuf::from(package.manifest_path.parent().context("Package manifest path has no parent")?);
            
            let relative_path = pathdiff::diff_paths(&package_path, &current_project_root)
                .unwrap_or_else(|| package_path.clone());
            
            crate_data.push(CrateInfo { // Use CrateInfo directly
                name: package.name.leak(),
                path: relative_path.to_string_lossy().into_owned().leak(),
            });
        }
    }

    let mut data_entries = Vec::new();
    for info in &crate_data {
        data_entries.push(format!(
            r#"CrateInfo {{ name: "{}", path: "{}" }}"#, // No .to_string()
            info.name,
            info.path
        ));
    }

    let serialized_data = format!(
        "const CARGO_TREE_DATA: &[CrateInfo] = &[
    {}
];",
        data_entries.join(",\n    ")
    );

    fs::write(&dest_path, serialized_data)?;

    Ok(())
}
