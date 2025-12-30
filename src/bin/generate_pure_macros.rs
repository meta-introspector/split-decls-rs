use std::collections::HashMap;
use std::fs;
use serde_json;

fn main() {
    println!("🔧 Generating pure Rust macro file");
    
    // Load recursive dependencies
    let recursive_data = fs::read_to_string("recursive_dependencies.json")
        .expect("Failed to read recursive_dependencies.json");
    
    let recursive_json: serde_json::Value = serde_json::from_str(&recursive_data)
        .expect("Failed to parse recursive_dependencies.json");
    
    // Load name index for file paths
    let name_index: HashMap<String, String> = serde_json::from_str(
        &fs::read_to_string("name_index.json").expect("Failed to read name_index.json")
    ).expect("Failed to parse name_index.json");
    
    if let Some(resolved_terms) = recursive_json.get("resolved_terms").and_then(|v| v.as_array()) {
        // Write pure Rust macros directly
        write_import_macros(&resolved_terms, &name_index);
        write_mkbin_macro(&resolved_terms, &name_index);
        
        println!("✅ Generated pure Rust macros for {} dependencies", resolved_terms.len());
    }
}

fn write_import_macros(resolved_terms: &[serde_json::Value], name_index: &HashMap<String, String>) {
    let mut file = fs::File::create("src/pure_macros.rs").expect("Failed to create pure_macros.rs");
    use std::io::Write;
    
    // Deduplicate by file path to avoid multiple macros for same file
    let mut path_to_name: HashMap<String, String> = HashMap::new();
    
    for term in resolved_terms {
        if let Some(term_str) = term.as_str() {
            if let Some(file_path) = name_index.get(term_str) {
                let corrected_path = if file_path.starts_with("output2/") {
                    format!("../../{}", file_path)
                } else {
                    file_path.to_string()
                };
                
                // Only create macro if we haven't seen this file path before
                if !path_to_name.contains_key(&corrected_path) {
                    let safe_name = term_str.replace("-", "_").replace(":", "_").replace("#", "_");
                    path_to_name.insert(corrected_path.clone(), safe_name.clone());
                    
                    writeln!(file, "macro_rules! import_{} {{", safe_name).unwrap();
                    writeln!(file, "    () => {{ include!(\"{}\"); }};", corrected_path).unwrap();
                    writeln!(file, "}}\n").unwrap();
                }
            }
        }
    }
}

fn write_mkbin_macro(resolved_terms: &[serde_json::Value], name_index: &HashMap<String, String>) {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("src/pure_macros.rs")
        .expect("Failed to open pure_macros.rs");
    use std::io::Write;
    
    writeln!(file, "macro_rules! mkbin {{").unwrap();
    writeln!(file, "    () => {{").unwrap();
    
    for term in resolved_terms {
        if let Some(term_str) = term.as_str() {
            if name_index.contains_key(term_str) {
                let safe_name = term_str.replace("-", "_").replace(":", "_").replace("#", "_");
                writeln!(file, "        mod {} {{", safe_name).unwrap();
                writeln!(file, "            println!(\"📦 Loading: {}\");", term_str).unwrap();
                writeln!(file, "            import_{}!();", safe_name).unwrap();
                writeln!(file, "        }}").unwrap();
            }
        }
    }
    
    writeln!(file, "    }};").unwrap();
    writeln!(file, "}}").unwrap();
}
