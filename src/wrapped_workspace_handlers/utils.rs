use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use toml::{Table, Value};

use crate::SplitDeclsConfig;
use crate::path_diff;


pub fn collect_and_format_workspace_dependencies(
    global_config: &SplitDeclsConfig,
    output_dir: &Path,
    scan_root: &Path,
) -> Result<HashMap<String, Value>> {
    let mut workspace_deps = HashMap::new();

    // Add dependencies from global_config
    for (dep_name, dep_value) in global_config.workspace_dependencies.iter() {
        let mut dep_value_clone = dep_value.clone();
        if let Some(dep_table) = dep_value_clone.as_table_mut() {
            if let Some(path_value) = dep_table.get("path") {
                if let Some(path_str) = path_value.as_str() {
                    let absolute_dep_path = scan_root.join(path_str);
                    let relative_path = path_diff::path_diff(output_dir, &absolute_dep_path)
                        .context(format!("Failed to calculate relative path for global workspace dependency '{}'", dep_name))?;
                    dep_table.insert("path".to_string(), Value::String(relative_path.display().to_string()));
                }
            }
            // Check if the original dependency (from global_config) had a 'path' field
            // and if it points to a local crate that will be wrapped.
            let mut is_local_path_dep = false;
            if let Some(dep_table_orig) = dep_value.as_table() { // Use original dep_value to check path
                if dep_table_orig.contains_key("path") {
                    is_local_path_dep = true;
                }
            }

            if is_local_path_dep {
                if let Some(dep_table_mut) = dep_value_clone.as_table_mut() {
                    // This is a local path dependency, so its path in the generated workspace.dependencies
                    // should point to the wrapped version of itself within output2/.
                    let wrapped_dep_name_for_path = format!("wrapped-{}", dep_name);
                    dep_table_mut.insert("path".to_string(), Value::String(wrapped_dep_name_for_path));
                    // Ensure 'workspace = true' is not propagated.
                    dep_table_mut.remove("workspace");
                } else {
                    // This case should ideally not happen for path dependencies (they are usually tables).
                    // If it does, it implies a malformed original Cargo.toml or unexpected structure.
                    eprintln!("Warning: Local path dependency '{}' does not have a table value. Skipping path rewriting.", dep_name);
                }
            } else {
                // For non-local path dependencies (e.g., versioned dependencies from crates.io),
                // just ensure 'workspace = true' is removed if present.
                if let Some(dep_table_mut) = dep_value_clone.as_table_mut() {
                    dep_table_mut.remove("workspace");
                }
            }
        workspace_deps.insert(dep_name.clone(), dep_value_clone);
    }

    // Add hardcoded introspector_decl2_macros if not already present
    let hardcoded_dep_name = "introspector_decl2_macros".to_string();
    if !workspace_deps.contains_key(&hardcoded_dep_name) {
        let mut hardcoded_dep_table = Table::new();
        hardcoded_dep_table.insert("path".to_string(), Value::String("../../submodules/patch-build-rs/introspector_decl2_macros".to_string()));
        workspace_deps.insert(hardcoded_dep_name, Value::Table(hardcoded_dep_table));
    }

    Ok(workspace_deps)
}
