use std::collections::{HashMap, HashSet};
use std::fs;
use std::env;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct ModuleAnalysis {
    defines: HashMap<String, Vec<String>>,
    uses: HashMap<String, Vec<String>>,
    providers: HashMap<String, Vec<String>>,
    consumers: HashMap<String, Vec<String>>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <routine_name_or_path>", args[0]);
        std::process::exit(1);
    }
    
    let target = &args[1];
    
    let analysis: ModuleAnalysis = serde_json::from_str(
        &fs::read_to_string("module_analysis.json").expect("module_analysis.json not found")
    ).expect("Failed to parse module_analysis.json");
    
    // Find matching files
    let matching_files: Vec<&String> = analysis.uses.keys()
        .filter(|path| path.contains(target))
        .collect();
    
    if matching_files.is_empty() {
        println!("❌ No files found matching: {}", target);
        return;
    }
    
    for file in matching_files {
        println!("🎯 Dependency tree for: {}", file);
        let mut visited = HashSet::new();
        print_dependency_tree(&analysis, file, 0, &mut visited);
        println!();
    }
}

fn print_dependency_tree(
    analysis: &ModuleAnalysis,
    file: &str,
    depth: usize,
    visited: &mut HashSet<String>,
) {
    let indent = "  ".repeat(depth);
    println!("{}├─ {}", indent, file);
    
    if visited.contains(file) {
        println!("{}   (circular reference)", indent);
        return;
    }
    visited.insert(file.to_string());
    
    if let Some(uses) = analysis.uses.get(file) {
        for symbol in uses {
            if let Some(providers) = analysis.providers.get(symbol) {
                for provider in providers {
                    if provider != file {
                        print_dependency_tree(analysis, provider, depth + 1, visited);
                    }
                }
            }
        }
    }
    
    visited.remove(file);
}
