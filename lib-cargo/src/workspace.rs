use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;
use toml::Value;

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
    
    for entry in walkdir::WalkDir::new(dir).max_depth(1) {
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
