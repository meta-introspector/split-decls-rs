use clap::Parser;
use anyhow::{Context, Result};
use std::collections::{HashSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use toml::{self, Table, Value};
use serde::{Deserialize, Serialize};

// --- Structs for parsing RootCargo.toml ---
#[derive(Debug, Deserialize)]
struct RootCargoToml {
    workspace: Option<RootWorkspace>,
}

#[derive(Debug, Deserialize)]
struct RootWorkspace {
    members: Option<Vec<String>>,
    dependencies: Option<HashMap<String, WorkspaceDependency>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum WorkspaceDependency {
    Simple(String), // e.g., "0.1.0" or "*"
    Detailed(DetailedWorkspaceDependency),
}

#[derive(Debug, Deserialize)]
struct DetailedWorkspaceDependency {
    path: Option<String>,
    package: Option<String>, // For when the package name differs from the dependency key
    #[serde(flatten)]
    other: HashMap<String, Value>, // Capture other fields like "features"
}

// --- Structs for SplitDeclsConfig (from split-decls-types/src/lib.rs) ---
// Duplicated here to avoid circular dependency for this tool.
// In a real project, this would be a shared library.
#[derive(Debug, Default, Serialize, Deserialize)]
struct SplitDeclsConfig {
    #[serde(default)]
    wrapping: WrappingConfig,
    #[serde(default)]
    crate_path_overrides: HashMap<String, String>, // Changed from PathBuf for simpler TOML serialization
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

    // 1. Load RootCargo.toml
    let root_cargo_content = fs::read_to_string(&args.root_cargo_toml_path)
        .context(format!("Failed to read RootCargo.toml from {}", args.root_cargo_toml_path.display()))?;
    let root_data: RootCargoToml = toml::from_str(&root_cargo_content)
        .context(format!("Failed to parse RootCargo.toml from {}", args.root_cargo_toml_path.display()))?;

    // 2. Load existing split-decls-rs.toml
    let mut split_decls_config_content = fs::read_to_string(&args.split_decls_config_path).unwrap_or_default();
    let mut split_decls_data: SplitDeclsConfig = toml::from_str(&split_decls_config_content).unwrap_or_default();

    let mut existing_crates_to_wrap: HashSet<String> = split_decls_data.wrapping.crates.drain(..).collect();
    let mut existing_path_overrides: HashMap<String, String> = split_decls_data.crate_path_overrides.drain().collect();

    // Determine the root of the main workspace (where RootCargo.toml is)
    let workspace_root = args.root_cargo_toml_path.parent().unwrap_or_else(|| Path::new(""));

    // Function to calculate relative path
    let calculate_relative_path = |full_path: &str| -> String {
        let full_path_buf = PathBuf::from(full_path);
        let relative_path = pathdiff::diff_paths(&full_path_buf, &args.current_project_root).unwrap_or(full_path_buf);
        relative_path.to_str().unwrap_or(full_path).to_string()
    };


    // Process [workspace.members]
    if let Some(workspace) = root_data.workspace {
        if let Some(members) = workspace.members {
            for member_path_str in members {
                let full_member_path = workspace_root.join(&member_path_str);
                let crate_name = full_member_path.file_name().and_then(|s| s.to_str()).unwrap_or_default().to_string();
                
                // Special handling for the current project itself
                if member_path_str == "submodules/split-decls-rs" {
                    existing_crates_to_wrap.insert("split-decls-rs".to_string());
                    existing_path_overrides.insert("split-decls-rs".to_string(), ".".to_string());
                    continue;
                }

                // Skip entries that are directly referring to split-decls-rs's own internal crates,
                // as those are handled by split-decls-rs's own Cargo.toml or explicit overrides.
                if member_path_str.starts_with("submodules/split-decls-rs/") {
                    continue;
                }

                existing_crates_to_wrap.insert(crate_name.clone());
                existing_path_overrides.insert(crate_name, calculate_relative_path(&full_member_path.to_string_lossy()));
            }
        }

        // Process [workspace.dependencies] with 'path'
        if let Some(dependencies) = workspace.dependencies {
            for (dep_key, dep_value) in dependencies {
                if let WorkspaceDependency::Detailed(detailed_dep) = dep_value {
                    if let Some(path_str) = detailed_dep.path {
                        let full_dep_path = workspace_root.join(&path_str);
                        let crate_name = detailed_dep.package.unwrap_or(dep_key);
                        
                        // Skip if it's pointing to split-decls-rs's internal crates.
                        if path_str.starts_with("submodules/split-decls-rs/") {
                            continue;
                        }

                        existing_crates_to_wrap.insert(crate_name.clone());
                        existing_path_overrides.insert(crate_name, calculate_relative_path(&full_dep_path.to_string_lossy()));
                    }
                }
            }
        }
    }

    // Sort crates for consistent output
    let mut sorted_crates: Vec<String> = existing_crates_to_wrap.into_iter().collect();
    sorted_crates.sort();
    split_decls_data.wrapping.crates = sorted_crates;
    
    // Sort overrides by key for consistent output
    let mut sorted_overrides: Vec<(String, String)> = existing_path_overrides.into_iter().collect();
    sorted_overrides.sort_by(|a, b| a.0.cmp(&b.0));
    split_decls_data.crate_path_overrides = sorted_overrides.into_iter().collect();

    // Explicitly set the correct path for proc-macro2 and others from current project's Cargo.toml
    split_decls_data.crate_path_overrides.insert("proc-macro2".to_string(), "submodules/proc-macro2".to_string());
    split_decls_data.crate_path_overrides.insert("split-decls-types".to_string(), "split-decls-types".to_string());
    split_decls_data.crate_path_overrides.insert("cargo-toml-generator-types".to_string(), "cargo-toml-generator-types".to_string());
    split_decls_data.crate_path_overrides.insert("cargo-toml-generator-macros".to_string(), "cargo-toml-generator-macros".to_string());
    split_decls_data.crate_path_overrides.insert("cargo-toml-parts".to_string(), "build_helpers/cargo_toml_parts".to_string());
    split_decls_data.crate_path_overrides.insert("pagerank_rs".to_string(), "submodules/pagerank_rs".to_string());


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
    /// Path to the RootCargo.toml file
    #[clap(long)]
    root_cargo_toml_path: PathBuf,

    /// Path to the split-decls-rs.toml configuration file to update
    #[clap(long)]
    split_decls_config_path: PathBuf,

    /// The current project root (where split-decls-rs Cargo.toml is located)
    #[clap(long)]
    current_project_root: PathBuf,
}

