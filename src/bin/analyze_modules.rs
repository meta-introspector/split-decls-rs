use split_decls_rs::extract_dependencies_from_macro_wrapped_code;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct ModuleAnalysis {
    defines: HashMap<String, Vec<String>>,  // module -> what it defines
    uses: HashMap<String, Vec<String>>,     // module -> what it uses
    providers: HashMap<String, Vec<String>>, // symbol -> who provides it
    consumers: HashMap<String, Vec<String>>, // symbol -> who uses it
}

fn main() {
    let mut analysis = ModuleAnalysis {
        defines: HashMap::new(),
        uses: HashMap::new(),
        providers: HashMap::new(),
        consumers: HashMap::new(),
    };
    
    let output2_path = Path::new("output2");
    if !output2_path.exists() {
        eprintln!("output2 directory not found");
        return;
    }
    
    process_directory(&output2_path, &mut analysis);
    build_relationships(&mut analysis);
    
    let analysis_json = serde_json::to_string_pretty(&analysis).unwrap();
    fs::write("module_analysis.json", analysis_json).unwrap();
    
    println!("Analyzed {} modules", analysis.defines.len());
    println!("Found {} unique symbols", analysis.providers.len());
    println!("Analysis saved to module_analysis.json");
}

fn process_directory(dir: &Path, analysis: &mut ModuleAnalysis) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                process_directory(&path, analysis);
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                if let Ok(content) = fs::read_to_string(&path) {
                    let module_path = path.to_string_lossy().to_string();
                    
                    // Extract what this module uses
                    let uses = extract_dependencies_from_macro_wrapped_code(&content);
                    if !uses.is_empty() {
                        analysis.uses.insert(module_path.clone(), uses.into_iter().collect());
                    }
                    
                    // Extract what this module defines
                    let defines = extract_definitions(&content);
                    if !defines.is_empty() {
                        analysis.defines.insert(module_path, defines);
                    }
                }
            }
        }
    }
}

fn extract_definitions(content: &str) -> Vec<String> {
    let mut definitions = Vec::new();
    
    // Look for macro definitions that define symbols
    let lines: Vec<&str> = content.lines().collect();
    let mut in_main_macro = false;
    let mut macro_name = String::new();
    
    for line in lines {
        let trimmed = line.trim();
        
        // Check for macro_rules! definitions
        if trimmed.starts_with("macro_rules!") && !trimmed.contains("deps") {
            if let Some(name) = trimmed.strip_prefix("macro_rules!").map(|s| s.trim()) {
                macro_name = name.to_string();
                in_main_macro = true;
            }
        }
        
        // End of macro
        if in_main_macro && trimmed == "}" {
            in_main_macro = false;
            if !macro_name.is_empty() {
                definitions.push(macro_name.clone());
                macro_name.clear();
            }
        }
        
        // Also look for direct symbol definitions in macro content
        if in_main_macro && (trimmed.contains("pub struct") || trimmed.contains("pub enum") || 
                            trimmed.contains("pub trait") || trimmed.contains("pub fn")) {
            // Extract symbol names from definitions
            if let Some(symbol) = extract_symbol_name(trimmed) {
                definitions.push(symbol);
            }
        }
    }
    
    definitions
}

fn extract_symbol_name(line: &str) -> Option<String> {
    let line = line.trim();
    
    if let Some(rest) = line.strip_prefix("pub struct") {
        return rest.trim().split_whitespace().next().map(|s| s.to_string());
    }
    if let Some(rest) = line.strip_prefix("pub enum") {
        return rest.trim().split_whitespace().next().map(|s| s.to_string());
    }
    if let Some(rest) = line.strip_prefix("pub trait") {
        return rest.trim().split_whitespace().next().map(|s| s.to_string());
    }
    if let Some(rest) = line.strip_prefix("pub fn") {
        return rest.trim().split('(').next().map(|s| s.trim().to_string());
    }
    
    None
}

fn build_relationships(analysis: &mut ModuleAnalysis) {
    // Build provider map: symbol -> modules that define it
    for (module, symbols) in &analysis.defines {
        for symbol in symbols {
            analysis.providers.entry(symbol.clone())
                .or_insert_with(Vec::new)
                .push(module.clone());
        }
    }
    
    // Build consumer map: symbol -> modules that use it
    for (module, symbols) in &analysis.uses {
        for symbol in symbols {
            analysis.consumers.entry(symbol.clone())
                .or_insert_with(Vec::new)
                .push(module.clone());
        }
    }
}
