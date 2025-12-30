use std::collections::HashMap;
use std::fs;
use serde_json;

fn main() {
    println!("🔧 Generating real import macros from JSON");
    
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
        let mut macro_content = String::new();
        
        // Generate import macro for each dependency
        for term in resolved_terms {
            if let Some(term_str) = term.as_str() {
                if let Some(file_path) = name_index.get(term_str) {
                    let safe_name = term_str.replace("-", "_").replace(":", "_").replace("#", "_");
                    // Fix path to be relative to project root when called from src/bin/
                    let corrected_path = if file_path.starts_with("output2/") {
                        format!("../../{}", file_path)
                    } else {
                        file_path.to_string()
                    };
                    macro_content.push_str(&format!(
                        "macro_rules! import_{} {{\n    () => {{\n        include!(\"{}\");\n    }};\n}}\n\n",
                        safe_name, corrected_path
                    ));
                }
            }
        }
        
        // Generate mkbin macro with optional override parameters and per-module overrides
        macro_content.push_str("macro_rules! mkbin {\n");
        macro_content.push_str("    () => {\n        mkbin!({}, {});\n    };\n");
        macro_content.push_str("    ($overrides:tt) => {\n        mkbin!($overrides, {});\n    };\n");
        macro_content.push_str("    ($overrides:tt, $module_overrides:tt) => {\n");
        
        // Define all the imports wrapped in modules with overrides and hook matching
        for term in resolved_terms {
            if let Some(term_str) = term.as_str() {
                if let Some(file_path) = name_index.get(term_str) {
                    let safe_name = term_str.replace("-", "_").replace(":", "_").replace("#", "_");
                    // Fix path to be relative to project root when called from src/bin/
                    let corrected_path = if file_path.starts_with("output2/") {
                        format!("../../{}", file_path)
                    } else {
                        file_path.to_string()
                    };
                    macro_content.push_str(&format!(
                        "        mod {} {{\n            $overrides\n            mkbin_override!({}, $module_overrides);\n            \n            // Hook matching for this declaration\n            macro_rules! mkdeclfn {{\n                ($name:ident) => {{\n                    if stringify!($name) == \"{}\" {{\n                        callback_{}!();\n                    }}\n                }};\n            }}\n            \n            include!(\"{}\");\n        }}\n",
                        safe_name, safe_name, term_str, safe_name, corrected_path
                    ));
                }
            }
        }
        
        macro_content.push_str("    };\n}\n\n");
        
        // Add helper macro for module-specific overrides
        macro_content.push_str("macro_rules! mkbin_override {\n");
        macro_content.push_str("    ($module:ident, { $($mod_name:ident => { $($override:tt)* }),* }) => {\n");
        macro_content.push_str("        mkbin_override_inner!($module, $($mod_name => { $($override)* }),*);\n");
        macro_content.push_str("    };\n");
        macro_content.push_str("    ($module:ident, {}) => {};\n");
        macro_content.push_str("}\n\n");
        
        macro_content.push_str("macro_rules! mkbin_override_inner {\n");
        macro_content.push_str("    ($module:ident, $module:ident => { $($override:tt)* }, $($rest:tt)*) => {\n");
        macro_content.push_str("        $($override)*\n");
        macro_content.push_str("    };\n");
        macro_content.push_str("    ($module:ident, $other:ident => { $($override:tt)* }, $($rest:tt)*) => {\n");
        macro_content.push_str("        mkbin_override_inner!($module, $($rest)*);\n");
        macro_content.push_str("    };\n");
        macro_content.push_str("    ($module:ident,) => {};\n");
        macro_content.push_str("}\n");
        
        fs::write("import_macros.rs", &macro_content)
            .expect("Failed to write import_macros.rs");
        
        println!("✅ Generated import_macros.rs with mkbin!() macro");
        println!("📝 Usage: include!(\"import_macros.rs\"); mkbin!();");
    }
}
