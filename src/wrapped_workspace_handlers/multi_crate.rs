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

    // Use the new `wrapping.crates` list to drive the process
    for crate_name in &global_config.wrapping.crates {
        let wrapped_crate_name = format!("wrapped-{}", crate_name);
        workspace_members_content.push(format!("\"{}\"", wrapped_crate_name));

        // Find the Cargo.toml for the crate. We assume it's in a directory with the same name.
        let cargo_toml_path = scan_root.join(crate_name).join("Cargo.toml");

        if !cargo_toml_path.exists() {
             // Look in submodules folder as a fallback
            let submodule_cargo_toml_path = scan_root.join("submodules").join(crate_name).join("Cargo.toml");
            if !submodule_cargo_toml_path.exists() {
                return Err(anyhow::anyhow!("Could not find Cargo.toml for crate '{}'", crate_name));
            }
             generate_wrapped_crate::generate_wrapped_crate(
                output_dir,
                crate_name,
                &submodule_cargo_toml_path.parent().unwrap().to_path_buf(),
                global_config,
                patch_config,
                dry_run,
            )?;
        } else {
             generate_wrapped_crate::generate_wrapped_crate(
                output_dir,
                crate_name,
                &cargo_toml_path.parent().unwrap().to_path_buf(),
                global_config,
                patch_config,
                dry_run,
            )?;
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
