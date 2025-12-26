use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;
use toml::Value;

/// Collect all dependencies from a manifest
pub fn collect_dependencies(manifest: &Value) -> HashMap<String, Value> {
    let mut deps = HashMap::new();
    
    for section in ["dependencies", "dev-dependencies", "build-dependencies"] {
        if let Some(section_deps) = manifest.get(section).and_then(|v| v.as_table()) {
            for (name, value) in section_deps {
                // Check if this dependency has a package attribute (alias)
                if let Some(table) = value.as_table() {
                    if let Some(package_name) = table.get("package").and_then(|v| v.as_str()) {
                        // Use the package name as the key, not the dependency name
                        deps.insert(package_name.to_string(), value.clone());
                    } else {
                        deps.insert(name.clone(), value.clone());
                    }
                } else {
                    deps.insert(name.clone(), value.clone());
                }
            }
        }
    }
    
    deps
}

/// Convert path dependencies to workspace format
pub fn convert_to_workspace_deps(
    deps: &HashMap<String, Value>,
    submodules_path: &Path,
) -> HashMap<String, Value> {
    let mut workspace_deps = HashMap::new();
    
    for (name, value) in deps {
        if let Some(table) = value.as_table() {
            if table.contains_key("path") {
                // Convert to relative path from workspace root
                let mut new_value = table.clone();
                if let Some(path_val) = new_value.get_mut("path") {
                    if let Some(path_str) = path_val.as_str() {
                        let relative_path = if path_str.contains("/submodules/") {
                            let parts: Vec<&str> = path_str.split("/submodules/").collect();
                            format!("../submodules/{}", parts[1])
                        } else {
                            format!("../{}", name)
                        };
                        *path_val = Value::String(relative_path);
                    }
                }
                workspace_deps.insert(name.clone(), Value::Table(new_value));
            } else {
                workspace_deps.insert(name.clone(), value.clone());
            }
        } else {
            workspace_deps.insert(name.clone(), value.clone());
        }
    }
    
    workspace_deps
}
