use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use toml::{Table, Value};
use walkdir::WalkDir; // Needed for original `else` block logic

use crate::path_diff;
use crate::find_all_cargo_tomls::find_all_cargo_tomls;
use crate::extract_crate_info_simple::extract_crate_info_simple; // Assuming these are in crate root
use crate::patch_config;
use split_decls_types::SplitDeclsConfig;
use crate::generate_wrapped_crate;
use super::utils::{collect_and_format_workspace_dependencies, format_toml_value_for_dependency_string}; // Use the new helper


pub fn handle_multi_crate_wrapping(
    output_dir: &Path,
    patch_config: &patch_config::PatchConfig,
    global_config: &SplitDeclsConfig,
    scan_root: &Path,
    dry_run: bool,
    verbose: bool,
) -> Result<(String, Vec<crate::eager_splitter::ModuleNotFoundReport>)> { // Changed return type
    let mut final_cargo_toml_content = String::new();
    let mut workspace_members_content = Vec::new();
    let mut workspace_dependencies_content_str = String::new();
    let mut patch_crates_io_content_str = String::new();
    let mut all_collected_errors: Vec<crate::eager_splitter::ModuleNotFoundReport> = Vec::new(); // Initialize error collector

    // Collect all workspace dependencies using the helper
    let consolidated_workspace_deps_map = collect_and_format_workspace_dependencies(
        global_config, output_dir, scan_root
    )?;
    for (dep_name, dep_value) in consolidated_workspace_deps_map.iter() {
        workspace_dependencies_content_str.push_str(&format!("{} = {}
", dep_name, format_toml_value_for_dependency_string(dep_value)));
    }

    use rayon::prelude::*;
    let processed_crates: Vec<(Option<String>, Vec<crate::eager_splitter::ModuleNotFoundReport>)> = global_config.wrapping.crates.par_iter().map(|crate_name| {
        let mut found_cargo_toml_path: Option<PathBuf> = None;
        let mut crate_errors: Vec<crate::eager_splitter::ModuleNotFoundReport> = Vec::new(); // Per-crate error collector

        if let Some(overrides) = &global_config.crate_path_overrides {
            if let Some(override_path) = overrides.get(crate_name) {
                let path_buf = PathBuf::from(override_path);
                let candidate_path = if path_buf.is_absolute() {
                    path_buf.join("Cargo.toml")
                } else {
                    scan_root.join(override_path).join("Cargo.toml")
                };
                if candidate_path.exists() {
                    found_cargo_toml_path = Some(candidate_path);
                }
            }
        }

        let cargo_toml_path = if let Some(path) = found_cargo_toml_path {
            path
        } else {
            // Original logic for crates directly under scan_root
            let direct_path = scan_root.join(crate_name).join("Cargo.toml");
            if direct_path.exists() {
                direct_path
            } else {
                // Fallback to submodules folder
                let submodule_path = scan_root.join("submodules").join(crate_name).join("Cargo.toml");
                if submodule_path.exists() {
                    submodule_path
                } else {
                    eprintln!("Could not find Cargo.toml for crate '{}'", crate_name);
                    return (None, crate_errors); // Return empty errors for this crate
                }
            }
        };

        let result = generate_wrapped_crate::generate_wrapped_crate(
            output_dir,
            crate_name,
            &cargo_toml_path.parent().unwrap().to_path_buf(),
            global_config,
            patch_config,
            dry_run,
        );
        match result {
            Ok(errors) => {
                let wrapped_crate_name = format!("wrapped-{}", crate_name);
                (Some(format!("\"{}\"", wrapped_crate_name)), errors)
            },
            Err(e) => {
                eprintln!("Error processing crate {}: {}", crate_name, e);
                (None, crate_errors) // Return empty errors on processing failure
            }
        }
    }).collect();

    for (member, errors) in processed_crates {
        if let Some(m) = member {
            workspace_members_content.push(m);
        }
        all_collected_errors.extend(errors); // Aggregate errors
    }
    
    eprintln!("DEBUG: workspace_dependencies_content_str:\n{}", workspace_dependencies_content_str);
    // Construct final workspace Cargo.toml content
    final_cargo_toml_content = format!(
        r#"[workspace]
resolver = "2"
members = [
    {}
]

[workspace.dependencies]
{}"#,
            workspace_members_content.join(",\n    "),
            workspace_dependencies_content_str
        );
        if !patch_crates_io_content_str.is_empty() {
            final_cargo_toml_content.push_str("\n[patch.crates-io]\n");
            final_cargo_toml_content.push_str(&patch_crates_io_content_str);
        }
    Ok((final_cargo_toml_content, all_collected_errors)) // Return tuple
}
