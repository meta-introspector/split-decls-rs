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
    cargo_only: bool,
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
    
    println!("🔍 PROCESSING {} CRATES FROM CONFIG:", global_config.wrapping.crates.len());
    println!("📂 SCAN ROOT DIRECTORY: {}", scan_root.display());
    for (i, crate_name) in global_config.wrapping.crates.iter().enumerate() {
        println!("   {}: {}", i + 1, crate_name);
    }
    
    let processed_crates: Vec<(Option<String>, Vec<crate::eager_splitter::ModuleNotFoundReport>)> = global_config.wrapping.crates.par_iter().map(|crate_name| {
        println!("🚀 STARTING CRATE: {}", crate_name);
        let mut found_cargo_toml_path: Option<PathBuf> = None;
        let mut crate_errors: Vec<crate::eager_splitter::ModuleNotFoundReport> = Vec::new(); // Per-crate error collector

        if let Some(overrides) = &global_config.crate_path_overrides {
            println!("   🔍 Checking path overrides for: {}", crate_name);
            if let Some(override_path) = overrides.get(crate_name) {
                println!("   ✅ Found override path: {}", override_path.display());
                let path_buf = PathBuf::from(override_path);
                let candidate_path = if path_buf.is_absolute() {
                    path_buf.join("Cargo.toml")
                } else {
                    scan_root.join(override_path).join("Cargo.toml")
                };
                println!("   📄 Checking candidate path: {}", candidate_path.display());
                if candidate_path.exists() {
                    println!("   ✅ Override path exists: {}", candidate_path.display());
                    found_cargo_toml_path = Some(candidate_path);
                } else {
                    println!("   ❌ Override path does not exist: {}", candidate_path.display());
                }
            } else {
                println!("   ⏭️  No override found for: {}", crate_name);
            }
        }

        let cargo_toml_path = if let Some(path) = found_cargo_toml_path {
            println!("   ✅ Using found path: {}", path.display());
            path
        } else {
            println!("   🔍 No override found, trying default paths for: {}", crate_name);
            // Original logic for crates directly under scan_root
            let direct_path = scan_root.join(crate_name).join("Cargo.toml");
            println!("   📄 Checking direct path: {}", direct_path.display());
            if direct_path.exists() {
                println!("   ✅ Direct path exists: {}", direct_path.display());
                direct_path
            } else {
                println!("   ❌ Direct path does not exist: {}", direct_path.display());
                // Fallback to submodules folder
                let submodule_path = scan_root.join("submodules").join(crate_name).join("Cargo.toml");
                println!("   📄 Checking submodule path: {}", submodule_path.display());
                if submodule_path.exists() {
                    println!("   ✅ Submodule path exists: {}", submodule_path.display());
                    submodule_path
                } else {
                    println!("   ❌ Submodule path does not exist: {}", submodule_path.display());
                    eprintln!("❌ Could not find Cargo.toml for crate '{}'", crate_name);
                    return (None, crate_errors); // Return empty errors for this crate
                }
            }
        };

        println!("   🔧 Calling generate_wrapped_crate for: {}", crate_name);
        println!("      📂 Output dir: {}", output_dir.display());
        println!("      📄 Cargo.toml: {}", cargo_toml_path.display());
        println!("      📁 Crate root: {}", cargo_toml_path.parent().unwrap().display());
        
        let result = generate_wrapped_crate::generate_wrapped_crate(
            output_dir,
            crate_name,
            &cargo_toml_path.parent().unwrap().to_path_buf(),
            global_config,
            patch_config,
            dry_run,
            cargo_only,
        );
        match result {
            Ok(errors) => {
                let wrapped_crate_name = format!("wrapped-{}", crate_name);
                println!("   ✅ SUCCESS: Generated {}", wrapped_crate_name);
                (Some(format!("\"{}\"", wrapped_crate_name)), errors)
            },
            Err(e) => {
                eprintln!("❌ ERROR processing crate {}: {}", crate_name, e);
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
    // Since all crates are wrapped, generate [patch.crates-io] entries for all of them
    for crate_name in &global_config.wrapping.crates {
        let mut patched_dep_table = Table::new();
        patched_dep_table.insert("path".to_string(), Value::String(format!("wrapped-{}", crate_name)));
        patch_crates_io_content_str.push_str(&format!(
            "{} = {}\n",
            crate_name,
            format_toml_value_for_dependency_string(&Value::Table(patched_dep_table))
        ));
    }
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
