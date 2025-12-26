use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;
use toml::Value;

#[derive(Debug, Clone)]
pub struct DependencyMetadata {
    pub name: String,
    pub value: Value,
    pub default_features: Option<bool>,
    pub features: Vec<String>,
    pub optional: bool,
}

/// Collect all dependencies from a manifest with metadata
pub fn collect_dependencies_with_metadata(manifest: &Value) -> HashMap<String, DependencyMetadata> {
    let mut deps = HashMap::new();
    
    for section in ["dependencies", "dev-dependencies", "build-dependencies"] {
        if let Some(section_deps) = manifest.get(section).and_then(|v| v.as_table()) {
            for (name, value) in section_deps {
                let metadata = extract_dependency_metadata(name, value);
                let key = metadata.name.clone();
                deps.insert(key, metadata);
            }
        }
    }
    
    deps
}

/// Extract metadata from a dependency value
fn extract_dependency_metadata(name: &str, value: &Value) -> DependencyMetadata {
    let mut metadata = DependencyMetadata {
        name: name.to_string(),
        value: value.clone(),
        default_features: None,
        features: Vec::new(),
        optional: false,
    };
    
    if let Some(table) = value.as_table() {
        // Check for package alias
        if let Some(package_name) = table.get("package").and_then(|v| v.as_str()) {
            metadata.name = package_name.to_string();
        }
        
        // Extract default-features
        if let Some(default_features) = table.get("default-features").and_then(|v| v.as_bool()) {
            metadata.default_features = Some(default_features);
        }
        
        // Extract features
        if let Some(features) = table.get("features").and_then(|v| v.as_array()) {
            metadata.features = features.iter()
                .filter_map(|f| f.as_str().map(|s| s.to_string()))
                .collect();
        }
        
        // Extract optional
        if let Some(optional) = table.get("optional").and_then(|v| v.as_bool()) {
            metadata.optional = optional;
        }
    }
    
    metadata
}

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

/// Convert path dependencies to workspace format using metadata
pub fn convert_to_workspace_deps_with_metadata(
    deps_metadata: &HashMap<String, DependencyMetadata>,
    submodules_path: &Path,
) -> HashMap<String, Value> {
    let mut workspace_deps = HashMap::new();
    
    for (name, metadata) in deps_metadata {
        if let Some(table) = metadata.value.as_table() {
            if table.contains_key("path") {
                let mut new_table = toml::Table::new();
                
                // Set path
                if let Some(path_val) = table.get("path").and_then(|v| v.as_str()) {
                    let relative_path = if path_val.contains("/submodules/") {
                        let parts: Vec<&str> = path_val.split("/submodules/").collect();
                        format!("../submodules/{}", parts[1])
                    } else {
                        format!("wrapped-{}", name)
                    };
                    new_table.insert("path".to_string(), Value::String(relative_path));
                }
                
                // Preserve default-features if specified
                if let Some(default_features) = metadata.default_features {
                    new_table.insert("default-features".to_string(), Value::Boolean(default_features));
                }
                
                // Preserve features if any
                if !metadata.features.is_empty() {
                    let features_array = metadata.features.iter()
                        .map(|f| Value::String(f.clone()))
                        .collect();
                    new_table.insert("features".to_string(), Value::Array(features_array));
                }
                
                // Preserve optional if true
                if metadata.optional {
                    new_table.insert("optional".to_string(), Value::Boolean(true));
                }
                
                workspace_deps.insert(name.clone(), Value::Table(new_table));
            } else {
                workspace_deps.insert(name.clone(), metadata.value.clone());
            }
        } else {
            workspace_deps.insert(name.clone(), metadata.value.clone());
        }
    }
    
    workspace_deps
}
