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

                // If it's a workspace dependency, it should be in workspace_dependencies_output_map.
                // Replace the reference in the current dependency group with `workspace = true`.
                if workspace_dependencies_output_map.contains_key(dep_name) {
                    output_map.insert(dep_name.clone(), toml::Value::Table(Table::from_iter(vec![("workspace".to_string(), Value::Boolean(true))] )) );
                } else if is_workspace_dep_ref {
                    // If it explicitly said `workspace = true` but is not in our collected workspace_dependencies,
                    // keep it as-is. This might lead to a Cargo error if not globally defined.
                    output_map.insert(dep_name.clone(), dep_value_clone);
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

    // Handle [workspace] section.
    // If the original Cargo.toml was not a workspace, add a new [workspace] section.
    // If it was already a workspace, we don't add a new [workspace] header, but we still write [workspace.dependencies].
    let mut workspace_section_exists_in_original = root_cargo_toml.get("workspace").is_some();
    let mut current_workspace_members = Vec::new();

    if workspace_section_exists_in_original {
        if let Some(workspace_table) = root_cargo_toml.get("workspace").and_then(|v| v.as_table()) {
            if let Some(members_array) = workspace_table.get("members").and_then(|v| v.as_array()) {
                for member in members_array {
                    if let Some(s) = member.as_str() {
                        current_workspace_members.push(format!("\"{}\"", s));
                    }
                }
            }
        }
    }

    // Ensure "." is a member for the single-crate scenario at output_dir
    if !current_workspace_members.contains(&"".to_string())
        && !current_workspace_members.contains(&".".to_string())
        && !current_workspace_members.contains(&"./".to_string())
        && !current_workspace_members.contains(&"..".to_string())
        && !current_workspace_members.contains(&"./src".to_string())
    {
        current_workspace_members.push(".".to_string());
    }
    
    if !workspace_section_exists_in_original {
        final_cargo_toml_content.push_str("\n[workspace]\n");
        final_cargo_toml_content.push_str("resolver = \"2\"\n");
        final_cargo_toml_content.push_str(&format!("members = [\n    {}\n]\n", current_workspace_members.join(",\n    ")));
    } else if !current_workspace_members.is_empty() {
        // If original had workspace and members were modified (e.g. added "."), then rewrite [workspace]
        final_cargo_toml_content.push_str("\n[workspace]\n");
        // Preserve resolver if exists, else default
        if let Some(resolver_val) = root_cargo_toml.get("workspace").and_then(|v| v.as_table()).and_then(|t| t.get("resolver")) {
             final_cargo_toml_content.push_str(&format!("resolver = {}\n", resolver_val.to_string()));
        } else {
            final_cargo_toml_content.push_str("resolver = \"2\"\n");
        }
        final_cargo_toml_content.push_str(&format!("members = [\n    {}\n]\n", current_workspace_members.join(",\n    ")));
    }


    // Add [workspace.dependencies] section if any were collected
    if !workspace_dependencies_output_map.is_empty() {
        final_cargo_toml_content.push_str("\n[workspace.dependencies]\n");
        for (key, value) in workspace_dependencies_output_map.iter() {
            final_cargo_toml_content.push_str(&format!("{} = {}\n", key, value.to_string()));
        }
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