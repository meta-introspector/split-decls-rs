use std::collections::HashMap;
use std::fs;
use serde_json;

fn main() {
    println!("🔧 Generating dependency import macros from recursive_dependencies.json");
    
    // Load recursive dependencies
    let recursive_data = fs::read_to_string("recursive_dependencies.json")
        .expect("Failed to read recursive_dependencies.json - run: cargo run --bin recursive_resolver");
    
    let recursive_json: serde_json::Value = serde_json::from_str(&recursive_data)
        .expect("Failed to parse recursive_dependencies.json");
    
    // Load name index for file paths
    let name_index: HashMap<String, String> = serde_json::from_str(
        &fs::read_to_string("name_index.json").expect("Failed to read name_index.json")
    ).expect("Failed to parse name_index.json");
    
    if let Some(resolved_terms) = recursive_json.get("resolved_terms").and_then(|v| v.as_array()) {
        println!("📊 Generating macros for {} resolved dependencies", resolved_terms.len());
        
        let mut macro_content = String::new();
        macro_content.push_str("// Auto-generated dependency import macros\n");
        macro_content.push_str("// Generated from recursive_dependencies.json\n\n");
        
        // Generate import_all_deps macro
        macro_content.push_str("macro_rules! import_all_deps {\n");
        macro_content.push_str("    () => {\n");
        
        for term in resolved_terms {
            if let Some(term_str) = term.as_str() {
                if let Some(file_path) = name_index.get(term_str) {
                    // Generate safe include for each dependency
                    macro_content.push_str(&format!(
                        "        // Import {}\n        include!(\"{}\");\n",
                        term_str, file_path
                    ));
                }
            }
        }
        
        macro_content.push_str("    };\n");
        macro_content.push_str("}\n\n");
        
        // Generate individual dependency macros that can be called
        macro_content.push_str("// Individual callable dependency macros\n");
        for term in resolved_terms {
            if let Some(term_str) = term.as_str() {
                if let Some(file_path) = name_index.get(term_str) {
                    let safe_name = term_str.replace("-", "_").replace(":", "_");
                    macro_content.push_str(&format!(
                        "macro_rules! call_{} {{\n    () => {{\n        // Load and execute {}\n        include!(\"{}\");\n        println!(\"🔧 Executed: {}\");\n    }};\n}}\n\n",
                        safe_name, term_str, file_path, term_str
                    ));
                }
            }
        }
        
        // Generate dependency execution macro that calls each one
        macro_content.push_str("macro_rules! execute_all_deps {\n");
        macro_content.push_str("    () => {\n");
        macro_content.push_str(&format!("        println!(\"🔧 Executing all {} dependencies\");\n", resolved_terms.len()));
        
        for (i, term) in resolved_terms.iter().enumerate() {
            if let Some(term_str) = term.as_str() {
                let safe_name = term_str.replace("-", "_").replace(":", "_");
                macro_content.push_str(&format!(
                    "        println!(\"  [{}/{}] Calling: {}\");\n        call_{}!();\n",
                    i + 1, resolved_terms.len(), term_str, safe_name
                ));
            }
        }
        
        macro_content.push_str("        println!(\"✅ All dependencies executed\");\n");
        macro_content.push_str("    };\n");
        macro_content.push_str("}\n");
        
        // Save the generated macros
        fs::write("dependency_macros.rs", &macro_content)
            .expect("Failed to write dependency_macros.rs");
        
        println!("✅ Generated dependency_macros.rs with {} callable macros", resolved_terms.len());
        println!("📝 Usage:");
        println!("   import_all_deps!();     // Import all dependencies");
        println!("   execute_all_deps!();    // Execute all dependencies");
        println!("   call_<name>!();         // Call specific dependency macro");
    }
}
