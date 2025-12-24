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
use super::utils::collect_and_format_workspace_dependencies; // Use the new helper


pub fn handle_multi_crate_wrapping(
    output_dir: &Path,
    patch_config: &patch_config::PatchConfig,
    global_config: &SplitDeclsConfig,
    scan_root: &Path,
    dry_run: bool,
    verbose: bool,
) -> Result<String> {
    let mut final_cargo_toml_content = String::new();
    let mut workspace_members_content = Vec::new();
    let mut workspace_dependencies_content_str = String::new();
    let mut patch_crates_io_content_str = String::new();

    // Collect all workspace dependencies using the helper
    let consolidated_workspace_deps_map = collect_and_format_workspace_dependencies(
        global_config, output_dir, scan_root
    )?;
    for (dep_name, dep_value) in consolidated_workspace_deps_map.iter() {
        workspace_dependencies_content_str.push_str(&format!("{} = {}
", dep_name, dep_value.to_string()));
    }


    if verbose {
        println!("Calling find_all_cargo_tomls in scan root: {}", scan_root.display());
    }
    if let Ok(cargo_tomls) = find_all_cargo_tomls(scan_root, verbose) {
        if verbose {
            println!("Found {} Cargo.toml files in scan root", cargo_tomls.len());
        }
        for cargo_path in cargo_tomls.iter() {
            if verbose {
                println!("  Extracting crate info from: {}", cargo_path.display());
            }
            if let Ok(Some(crate_info)) = extract_crate_info_simple(&cargo_path) {
                let wrapped_crate_name = format!("wrapped-{}", crate_info.name);
                // The member path should be the wrapped crate's directory relative to the output_dir
                let wrapped_crate_path_in_output = output_dir.join(&wrapped_crate_name);
                let relative_path_for_member = path_diff::path_diff(output_dir, &wrapped_crate_path_in_output)
                    .context(format!("Failed to calculate relative path for wrapped crate member '{}'", wrapped_crate_name))?;

                workspace_members_content.push(format!("\"{}\"", relative_path_for_member.display()));
                if verbose {
                    println!("    Added crate {} as workspace member and dependency candidate.", crate_info.name);
                }
                // Call generate_wrapped_crate for each discovered crate
                if verbose {
                    println!("  Calling generate_wrapped_crate for discovered crate '{}' at '{}'", crate_info.name, cargo_path.display());
                }
                generate_wrapped_crate::generate_wrapped_crate(
                    output_dir,
                    &crate_info.name, // Pass the actual crate name
                    &cargo_path.parent().unwrap().to_path_buf(), // Pass the directory of the Cargo.toml
                    global_config,
                    patch_config,
                    dry_run,
                )?;
                if verbose {
                    println!("  Finished generate_wrapped_crate for discovered crate: {}", crate_info.name);
                }
            }
        }
        if verbose {
            println!("Generated {} workspace members from scan root", workspace_members_content.len());
        }
    } else {
        if verbose {
            println!("Failed to find any Cargo.toml files in scan root: {}", scan_root.display());
        }
    }

    // Process generated_workspace_member from patch_config
    for member in &patch_config.generated_workspace_member {
        if verbose {
            println!("Processing generated workspace member: {}", member.name);
        }
        let wrapped_crate_name = format!("wrapped-{}", member.name);
        // The member path should be the wrapped crate's directory relative to the output_dir
        let wrapped_crate_path_in_output = output_dir.join(&wrapped_crate_name);
        let relative_path_for_member = path_diff::path_diff(output_dir, &wrapped_crate_path_in_output)
            .context(format!("Failed to calculate relative path for wrapped crate member '{}'", wrapped_crate_name))?;
        workspace_members_content.push(format!("\"{}\"", relative_path_for_member.display()));

        // Call generate_wrapped_crate for each member
        let original_crate_location = scan_root.join(&member.path);
        
        if verbose {
            println!("  Calling generate_wrapped_crate for member '{}' at '{}'", member.name, original_crate_location.display());
        }
        // This is where individual members are processed and their generated content written
        generate_wrapped_crate::generate_wrapped_crate(
            output_dir,
            &member.name,
            &original_crate_location,
            global_config,
            patch_config,
            dry_run,
        )?;
        if verbose {
            println!("  Finished generate_wrapped_crate for member: {}", member.name);
        }
    }

    // Process generated_workspace_dependency from patch_config
    for dep in &patch_config.generated_workspace_dependency {
        if verbose {
            println!("Processing generated workspace dependency: {}", dep.name);
        }
        let mut dep_string = format!("{} = {{ ", dep.name);

        let mut parts = Vec::new();

        if let Some(version) = &dep.version {
            parts.push(format!("version = \"{}\"", version));
        }

        if let Some(project_root_path) = &dep.project_root_path {
            let full_dep_path = scan_root.join(project_root_path);
            let relative_path = match path_diff::path_diff(output_dir, &full_dep_path) {
                Some(p) => p,
                None => anyhow::bail!(format!("Failed to calculate relative path for workspace dep '{}'", dep.name)),
            };
            parts.push(format!("path = \"{}\"", relative_path.display()));
        }

        if let Some(features) = &dep.features {
            parts.push(format!("features = [\"{}\"]", features.join("\", \"")));
        }

        if let Some(package) = &dep.package {
            parts.push(format!("package = \"{}\"", package));
        }

        dep_string.push_str(&parts.join(", "));
        dep_string.push_str(" }\n");

        if dep.is_patch.unwrap_or(false) {
            patch_crates_io_content_str.push_str(&dep_string);
        } else {
            // This will now be handled by collect_and_format_workspace_dependencies for consistency
            // We might need to ensure these are merged with the output of collect_and_format_workspace_dependencies
            // For now, these direct generated_workspace_dependency will be ignored if not patch
        }
    }
    
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
    Ok(final_cargo_toml_content)
}
