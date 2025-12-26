use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;
use toml::Value;
use walkdir::WalkDir;

/// Generate workspace Cargo.toml with members and dependencies
pub fn generate_workspace_toml(
    members: &[String],
    dependencies: &HashMap<String, Value>,
    output_path: &Path,
) -> Result<()> {
    let mut content = String::new();
    content.push_str("[workspace]\nresolver = \"2\"\nmembers = [\n");
    
    for member in members {
        content.push_str(&format!("    \"{}\",\n", member));
    }
    
    content.push_str("]\n\n[workspace.dependencies]\n");
    
    for (name, value) in dependencies {
        content.push_str(&format!("{} = {}\n", name, value));
    }
    
    std::fs::write(output_path, content)?;
    Ok(())
}

/// Collect all workspace members from a directory
pub fn collect_workspace_members(dir: &Path) -> Result<Vec<String>> {
    let mut members = Vec::new();
    
    for entry in WalkDir::new(dir).max_depth(1) {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() && path != dir {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("wrapped-") && path.join("Cargo.toml").exists() {
                    members.push(name.to_string());
                }
            }
        }
    }
    
    members.sort();
    Ok(members)
}

/// Scan all Cargo.toml files in workspace and collect their dependencies
pub fn collect_all_workspace_dependencies(workspace_dir: &Path) -> Result<HashMap<String, Value>> {
    let mut all_deps = HashMap::new();
    
    for entry in WalkDir::new(workspace_dir).max_depth(2) {
        let entry = entry?;
        let path = entry.path();
        
        if path.file_name() == Some(std::ffi::OsStr::new("Cargo.toml")) && path != workspace_dir.join("Cargo.toml") {
            if let Ok(manifest) = crate::manifest::read_manifest(path) {
                let deps = crate::dependencies::collect_dependencies(&manifest);
                for (name, _value) in deps {
                    // Special handling for alloc alias
                    if name == "alloc" {
                        let mut workspace_entry = toml::map::Map::new();
                        workspace_entry.insert("path".to_string(), Value::String("./wrapped-rustc-std-workspace-alloc".to_string()));
                        all_deps.insert(name, Value::Table(workspace_entry));
                    } else {
                        // Convert to workspace format - just set workspace = true for all deps
                        let mut workspace_entry = toml::map::Map::new();
                        workspace_entry.insert("workspace".to_string(), Value::Boolean(true));
                        all_deps.insert(name, Value::Table(workspace_entry));
                    }
                }
            }
        }
    }
    
    Ok(all_deps)
}
