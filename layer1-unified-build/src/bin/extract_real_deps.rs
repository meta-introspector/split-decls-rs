use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rust_dir = std::env::args().nth(1)
        .unwrap_or_else(|| "../../rust/compiler".to_string());
    
    println!("🔍 EXTRACTING REAL RUSTC CRATE DEPENDENCIES");
    println!("📁 Rust compiler directory: {}", rust_dir);
    println!("{}", "═".repeat(60));
    
    let dependencies = extract_crate_dependencies(&rust_dir)?;
    let build_order = calculate_build_order(&dependencies)?;
    
    println!("\n📊 REAL CRATE DEPENDENCIES:");
    println!("{:<30} {}", "Crate", "Dependencies");
    println!("{}", "─".repeat(80));
    
    for (crate_name, deps) in &dependencies {
        let dep_str = if deps.is_empty() {
            "none".to_string()
        } else {
            deps.iter().cloned().collect::<Vec<_>>().join(", ")
        };
        println!("{:<30} {}", crate_name, dep_str);
    }
    
    println!("\n🔄 BUILD ORDER:");
    for (level, crates) in build_order.iter().enumerate() {
        println!("Level {}: {}", level, crates.join(", "));
    }
    
    Ok(())
}

fn extract_crate_dependencies(rust_dir: &str) -> Result<HashMap<String, HashSet<String>>, Box<dyn std::error::Error>> {
    let mut dependencies = HashMap::new();
    
    for entry in fs::read_dir(rust_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let crate_name = entry.file_name().to_string_lossy().to_string();
            let cargo_toml = entry.path().join("Cargo.toml");
            
            if cargo_toml.exists() {
                let deps = parse_cargo_toml(&cargo_toml)?;
                dependencies.insert(crate_name, deps);
            }
        }
    }
    
    Ok(dependencies)
}

fn parse_cargo_toml(cargo_toml: &Path) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(cargo_toml)?;
    let mut deps = HashSet::new();
    
    let mut in_dependencies = false;
    
    for line in content.lines() {
        let line = line.trim();
        
        if line == "[dependencies]" {
            in_dependencies = true;
            continue;
        }
        
        if line.starts_with('[') && line != "[dependencies]" {
            in_dependencies = false;
            continue;
        }
        
        if in_dependencies && !line.is_empty() && !line.starts_with('#') {
            if let Some(dep_name) = extract_dependency_name(line) {
                if dep_name.starts_with("rustc_") {
                    deps.insert(dep_name);
                }
            }
        }
    }
    
    Ok(deps)
}

fn extract_dependency_name(line: &str) -> Option<String> {
    if let Some(eq_pos) = line.find('=') {
        let name = line[..eq_pos].trim();
        Some(name.to_string())
    } else {
        None
    }
}

fn calculate_build_order(dependencies: &HashMap<String, HashSet<String>>) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let mut levels = Vec::new();
    let mut remaining: HashSet<String> = dependencies.keys().cloned().collect();
    let mut built = HashSet::new();
    
    while !remaining.is_empty() {
        let mut current_level = Vec::new();
        
        for crate_name in &remaining {
            let deps = dependencies.get(crate_name).unwrap();
            if deps.iter().all(|dep| built.contains(dep) || !dependencies.contains_key(dep)) {
                current_level.push(crate_name.clone());
            }
        }
        
        if current_level.is_empty() {
            return Err("Circular dependency detected".into());
        }
        
        for crate_name in &current_level {
            remaining.remove(crate_name);
            built.insert(crate_name.clone());
        }
        
        levels.push(current_level);
    }
    
    Ok(levels)
}
