use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use toml::{Table, Value};

use crate::{add_generated_header}; use crate::path_diff;
use crate::patch_config;
use split_decls_types::SplitDeclsConfig; // Assuming split_decls_types is in crate root
use crate::wrapped_workspace_handlers::utils::collect_and_format_workspace_dependencies; // Use the new helper

pub fn handle_single_crate_wrapping(
    output_dir: &Path,
    patch_config: &patch_config::PatchConfig,
    global_config: &SplitDeclsConfig,
    scan_root: &Path,
    dry_run: bool, // Not used directly in this function, but kept for signature consistency
    verbose: bool,
    main_crate_cargo_toml_path: &Path,
    root_cargo_toml: &Table,
) -> Result<String> {
    let mut final_cargo_toml_content = String::new();

    // Construct [package] section
    if let Some(package_section) = root_cargo_toml.get("package").and_then(|v| v.as_table()) {
        final_cargo_toml_content.push_str("[package]\n");
        for (key, value) in package_section.iter() {
            // Skip workspace-related keys if they exist in the package section (unlikely but good practice)
            if key != "workspace" {
                final_cargo_toml_content.push_str(&format!("{} = {}\n", key, value.to_string()));
            }
        }
    } else {
        anyhow::bail!("No [package] section found in {}", main_crate_cargo_toml_path.display());
    }

    let mut package_deps_output = HashMap::new();
    let mut package_build_deps_output = HashMap::new();
    let mut package_dev_deps_output = HashMap::new();
    let mut package_patch_deps_output = HashMap::new();
    let workspace_dependencies_output_map = collect_and_format_workspace_dependencies(
        global_config, output_dir, scan_root
    )?;


    // Helper to process dependencies, determining if they should be moved to [workspace.dependencies]
    // or kept as direct dependencies.
    let mut process_dep_group = |deps_source: Option<&Table>, output_map: &mut HashMap<String, Value>| -> Result<()> {
        if let Some(deps_table) = deps_source {
            for (dep_name, dep_value) in deps_table.iter() {
                let mut dep_value_clone = dep_value.clone();
                let mut is_workspace_dep_ref = false;

                if let Some(dep_table) = dep_value_clone.as_table_mut() {
                    if dep_table.contains_key("workspace") && dep_table["workspace"].as_bool().unwrap_or(false) {
                        is_workspace_dep_ref = true;
                        // Remove `workspace = true` as it's implicit when referencing [workspace.dependencies]
                        dep_table.remove("workspace");
                    }
                }

                // Only use workspace = true for dependencies that are actually in workspace_dependencies_output_map
                // This prevents errors for external dependencies that aren't defined in [workspace.dependencies]
                if workspace_dependencies_output_map.contains_key(dep_name) {
                    output_map.insert(dep_name.clone(), toml::Value::Table(Table::from_iter(vec![("workspace".to_string(), Value::Boolean(true))] )) );
                } else if is_workspace_dep_ref {
                    // If it explicitly said `workspace = true` but is not in our collected workspace_dependencies,
                    // convert it back to a regular dependency to avoid Cargo errors
                    if let Some(original_dep) = deps_table.get(dep_name) {
                        output_map.insert(dep_name.clone(), original_dep.clone());
                    }
                }
                else {
                    // Regular direct dependency
                    if let Some(dep_table) = dep_value_clone.as_table_mut() {
                        if let Some(path_value) = dep_table.get("path") {
                            if let Some(path_str) = path_value.as_str() {
                                let absolute_dep_path = scan_root.join(path_str);
                                let relative_path = path_diff::path_diff(output_dir, &absolute_dep_path)
                                    .context(format!("Failed to calculate relative path for dependency '{}'", dep_name))?;
                                dep_table.insert("path".to_string(), Value::String(relative_path.display().to_string()));
                            }
                        }
                    }
                    output_map.insert(dep_name.clone(), dep_value_clone);
                }
            }
        }
        Ok(())
    };

    process_dep_group(root_cargo_toml.get("dependencies").and_then(|v| v.as_table()), &mut package_deps_output)?;
    process_dep_group(root_cargo_toml.get("build-dependencies").and_then(|v| v.as_table()), &mut package_build_deps_output)?;
    process_dep_group(root_cargo_toml.get("dev-dependencies").and_then(|v| v.as_table()), &mut package_dev_deps_output)?;
    // For patch dependencies, we handle them directly in the patch section
    if let Some(patch_section) = root_cargo_toml.get("patch").and_then(|v| v.as_table()) {
        if let Some(crates_io_patch) = patch_section.get("crates-io").and_then(|v| v.as_table()) {
            package_patch_deps_output.extend(crates_io_patch.clone());
        }
    }


    // Write the consolidated dependencies to final_cargo_toml_content
    if !package_deps_output.is_empty() {
        final_cargo_toml_content.push_str("\n[dependencies]\n");
        for (key, value) in package_deps_output.iter() {
            final_cargo_toml_content.push_str(&format!("{} = {}\n", key, value.to_string()));
        }
    }

    if !package_build_deps_output.is_empty() {
        final_cargo_toml_content.push_str("\n[build-dependencies]\n");
        for (key, value) in package_build_deps_output.iter() {
            final_cargo_toml_content.push_str(&format!("{} = {}\n", key, value.to_string()));
        }
    }

    if !package_dev_deps_output.is_empty() {
        final_cargo_toml_content.push_str("\n[dev-dependencies]\n");
        for (key, value) in package_dev_deps_output.iter() {
            final_cargo_toml_content.push_str(&format!("{} = {}\n", key, value.to_string()));
        }
    }

    // Add [workspace.dependencies] section if any were collected
    if !workspace_dependencies_output_map.is_empty() {
        // This section should only be added if it's part of a workspace.
        // If this function generates content for a single package,
        // workspace.dependencies should not be in its Cargo.toml.
        // However, if the output Cargo.toml is meant to be a workspace root
        // (which seems to be the case for output2/Cargo.toml),
        // then this content should be handled by the workspace generator,
        // not the single-crate generator.
        // For now, let's remove it from here to prevent duplication if a workspace
        // is being generated at the top level.
    }
    // Handle [[bin]] sections - direct copy of relevant parts, but adjust paths
    if let Some(bin_array) = root_cargo_toml.get("bin").and_then(|v| v.as_array()) {
        for bin_item in bin_array {
            if let Some(bin_table) = bin_item.as_table() {
                final_cargo_toml_content.push_str("\n[[bin]]\n");
                for (key, value) in bin_table.iter() {
                    if key == "path" {
                        if let Some(path_str) = value.as_str() {
                            let absolute_bin_path = scan_root.join(path_str);
                            let relative_bin_path = path_diff::path_diff(output_dir, &absolute_bin_path)
                                .context(format!("Failed to calculate relative path for bin '{}'", path_str))?;
                            final_cargo_toml_content.push_str(&format!("path = \"{}\"\n", relative_bin_path.display()));
                        }
                    } else {
                        final_cargo_toml_content.push_str(&format!("{} = {}\n", key, value.to_string()));
                    }
                }
            }
        }
    }

    // Handle [patch] sections
    if !package_patch_deps_output.is_empty() {
        final_cargo_toml_content.push_str("\n[patch.crates-io]\n");
        for (key, value) in package_patch_deps_output.iter() {
            final_cargo_toml_content.push_str(&format!("{} = {}\n", key, value.to_string()));
        }
    }
    Ok(final_cargo_toml_content)
}