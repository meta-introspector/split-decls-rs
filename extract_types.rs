use std::fs;
use std::collections::HashSet;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let file = "src/processed_rustc_abi_rustc_abi_src_callconv.rs";
    let content = fs::read_to_string(file)?;
    
    // Extract what types are actually used from reg module
    let used_types = extract_used_types(&content, "reg");
    
    println!("🔍 Types used from 'reg' module: {:?}", used_types);
    
    // Generate proper stub
    let stub = generate_module_stub("reg", &used_types);
    println!("📦 Generated stub:\n{}", stub);
    
    Ok(())
}

fn extract_used_types(content: &str, module_name: &str) -> Vec<String> {
    let mut used_types = Vec::new();
    
    for line in content.lines() {
        let trimmed = line.trim();
        
        // Look for "pub use module::{Type1, Type2}"
        if trimmed.starts_with("pub use ") && trimmed.contains(&format!("{}::", module_name)) {
            println!("🔍 Found use line: {}", trimmed);
            
            if let Some(types_part) = trimmed.split('{').nth(1) {
                if let Some(types_str) = types_part.split('}').next() {
                    println!("📦 Types string: '{}'", types_str);
                    
                    for type_name in types_str.split(',') {
                        let clean_name = type_name.trim();
                        if !clean_name.is_empty() {
                            used_types.push(clean_name.to_string());
                            println!("✅ Added type: '{}'", clean_name);
                        }
                    }
                }
            }
        }
    }
    
    used_types
}

fn generate_module_stub(module_name: &str, used_types: &[String]) -> String {
    let mut stub = format!("pub mod {} {{\n", module_name);
    
    for type_name in used_types {
        stub.push_str(&format!("    #[derive(Copy, Clone, Debug, PartialEq)]\n"));
        stub.push_str(&format!("    pub struct {};\n\n", type_name));
    }
    
    stub.push_str("}\n");
    stub
}
