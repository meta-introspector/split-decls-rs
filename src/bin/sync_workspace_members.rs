use anyhow::Result;
use std::path::Path;
use std::fs;
use toml_edit::{Document, Array, value};

fn main() -> Result<()> {
    let parent_cargo_path = Path::new("../../Cargo.toml");
    let current_dir = Path::new(".");
    
    println!("Syncing workspace members to parent Cargo.toml...");
    
    // Find all local crates that should be workspace members
    let local_crates = find_local_workspace_crates(current_dir)?;
    
    // Update parent Cargo.toml
    update_parent_workspace(&parent_cargo_path, &local_crates)?;
    
    println!("Successfully synced {} workspace members", local_crates.len());
    
    Ok(())
}

fn find_local_workspace_crates(dir: &Path) -> Result<Vec<String>> {
    let mut crates = Vec::new();
    
    // Look for immediate subdirectories with Cargo.toml
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            let cargo_toml = path.join("Cargo.toml");
            if cargo_toml.exists() {
                if let Some(name) = path.file_name() {
                    crates.push(name.to_string_lossy().to_string());
                }
            }
        }
    }
    
    crates.sort();
    Ok(crates)
}

fn update_parent_workspace(cargo_path: &Path, new_members: &[String]) -> Result<()> {
    let content = fs::read_to_string(cargo_path)?;
    let mut doc = content.parse::<toml_edit::Document>()?;
    
    // Get or create workspace table
    if !doc.contains_key("workspace") {
        doc["workspace"] = toml_edit::table();
    }
    
    // Get or create members array
    if !doc["workspace"].as_table().unwrap().contains_key("members") {
        doc["workspace"]["members"] = value(Array::new());
    }
    
    let members_array = doc["workspace"]["members"].as_array_mut().unwrap();
    
    // Add new members with submodules/split-decls-rs/ prefix
    for member in new_members {
        let full_path = format!("submodules/split-decls-rs/{}", member);
        
        // Check if already exists
        let exists = members_array.iter().any(|item| {
            item.as_str() == Some(&full_path)
        });
        
        if !exists {
            members_array.push(full_path);
            println!("Added workspace member: submodules/split-decls-rs/{}", member);
        }
    }
    
    // Write back to file
    fs::write(cargo_path, doc.to_string())?;
    
    Ok(())
}
