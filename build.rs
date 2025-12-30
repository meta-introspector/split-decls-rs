use std::fs;
use serde_json::Value;

fn main() {
    println!("cargo:rerun-if-changed=recursive_dependencies.json");
    
    // Read JSON and generate mkbin macro
    let json_content = fs::read_to_string("recursive_dependencies.json").unwrap();
    let deps: Value = serde_json::from_str(&json_content).unwrap();
    
    let mut macro_content = String::from("include!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/import_macros.rs\"));\n\nmacro_rules! mkbin {\n    () => {\n");
    
    if let Some(resolved) = deps["resolved_terms"].as_array() {
        for term in resolved.iter().take(10) {
            if let Some(name) = term.as_str() {
                macro_content.push_str(&format!("        mod {} {{\n", name));
                macro_content.push_str(&format!("            println!(\"Module {} loaded\");\n", name));
                macro_content.push_str(&format!("            import_{}!();\n", name));
                macro_content.push_str("        }\n");
            }
        }
    }
    
    macro_content.push_str("        println!(\"mkbin executed with modules\");\n");
    macro_content.push_str("    };\n}\n");
    
    fs::write("src/generated_mkbin.rs", macro_content).unwrap();
}
