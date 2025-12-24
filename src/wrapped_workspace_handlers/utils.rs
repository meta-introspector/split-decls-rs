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

    // 1. Add dependencies from global_config.workspace_dependencies
    for (dep_name, dep_value) in global_config.workspace_dependencies.iter() {
        let mut dep_value_clone = dep_value.clone();

        if let Some(dep_table) = dep_value_clone.as_table_mut() {
            // Adjust path for local dependencies to be relative to the output_dir
            if let Some(path_value) = dep_table.get("path") {
                if let Some(path_str) = path_value.as_str() {
                    let absolute_dep_path = scan_root.join(path_str);
                    if absolute_dep_path.exists() {
                        let relative_path = path_diff::path_diff(output_dir, &absolute_dep_path)
                            .context(format!("Failed to calculate relative path for global workspace dependency '{}'", dep_name))?;
                        dep_table.insert("path".to_string(), Value::String(relative_path.display().to_string()));
                    }
                    // For local deps, we also need to point to their wrapped version
                    let wrapped_dep_name = format!("wrapped-{}", dep_name);
                    dep_table.insert("path".to_string(), Value::String(wrapped_dep_name));

                }
            }
            dep_table.remove("workspace"); // Ensure workspace = true is not propagated here
        }
        workspace_deps.insert(dep_name.clone(), dep_value_clone);
    }

    // 2. Apply overrides from workspace_dependency_overrides
    for (dep_name, override_value) in &global_config.workspace_dependency_overrides {
        workspace_deps.insert(dep_name.clone(), override_value.clone());
    }
    
    // 3. Add hardcoded introspector_decl2_macros if not already present
    let hardcoded_dep_name = "introspector_decl2_macros".to_string();
    if !workspace_deps.contains_key(&hardcoded_dep_name) {
        let mut hardcoded_dep_table = Table::new();
        hardcoded_dep_table.insert("path".to_string(), Value::String("../../submodules/patch-build-rs/introspector_decl2_macros".to_string()));
        workspace_deps.insert(hardcoded_dep_name, Value::Table(hardcoded_dep_table));
    }

    Ok(workspace_deps)
}
