use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::Result;

fn main() -> Result<()> {
    println!("🦀 Evaluating rustc main from output2 split form");
    
    // Load the main rustc driver
    let main_path = "output2/wrapped-rustc_driver_impl/src/decls/main.rs";
    if !Path::new(main_path).exists() {
        println!("❌ Main rustc driver not found at {}", main_path);
        return Ok(());
    }
    
    let main_content = fs::read_to_string(main_path)?;
    println!("✅ Found rustc main driver");
    println!("📄 Content preview:");
    println!("{}", &main_content[..std::cmp::min(200, main_content.len())]);
    
    // Extract dependencies from the macro
    let deps = extract_dependencies(&main_content);
    println!("🔗 Found {} dependencies:", deps.len());
    for dep in &deps {
        println!("  - {}", dep);
    }
    
    // Try to resolve each dependency
    let mut resolved = HashMap::new();
    for dep in &deps {
        if let Some(path) = find_dependency_file(dep) {
            println!("✅ Resolved {}: {}", dep, path);
            resolved.insert(dep.clone(), path);
        } else {
            println!("❌ Could not resolve: {}", dep);
        }
    }
    
    println!("📊 Resolution summary: {}/{} dependencies resolved", 
             resolved.len(), deps.len());
    
    Ok(())
}

fn extract_dependencies(content: &str) -> Vec<String> {
    let mut deps = Vec::new();
    
    // Look for macro calls like TimePassesCallbacks!()
    for line in content.lines() {
        if let Some(start) = line.find("!()") {
            let before = &line[..start];
            if let Some(name_start) = before.rfind(|c: char| !c.is_alphanumeric() && c != '_') {
                let name = &before[name_start + 1..];
                if !name.is_empty() && name.chars().next().unwrap().is_uppercase() {
                    deps.push(name.to_string());
                }
            }
        }
    }
    
    deps
}

fn find_dependency_file(dep: &str) -> Option<String> {
    // Look for the dependency in output2
    let patterns = vec![
        format!("output2/wrapped-rustc_driver_impl/src/decls/{}.rs", dep),
        format!("output2/wrapped-rustc_driver_impl/src/decls/{}", dep),
    ];
    
    for pattern in patterns {
        if Path::new(&pattern).exists() {
            return Some(pattern);
        }
    }
    
    None
}
