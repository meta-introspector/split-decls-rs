use std::fs;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone)]
struct ModuleDecls {
    types: HashMap<String, String>, // type_name -> full_declaration
}

impl ModuleDecls {
    fn new() -> Self {
        Self { types: HashMap::new() }
    }
    
    fn merge(&mut self, other: ModuleDecls) {
        for (name, decl) in other.types {
            self.types.insert(name, decl);
        }
    }
    
    fn generate_stub(&self, module_name: &str) -> String {
        let mut stub = format!("pub mod {} {{\n", module_name);
        for decl in self.types.values() {
            stub.push_str(&format!("    {}\n", decl));
        }
        stub.push_str("}\n\n");
        stub
    }
}

fn main() -> Result<()> {
    // Get first few processed files
    let mut files = Vec::new();
    for entry in fs::read_dir("src")? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with("processed_") && name.ends_with(".rs") {
            files.push(name);
        }
    }
    files.sort();
    
    println!("🔄 Processing {} files progressively...", files.len().min(5));
    
    // Accumulate declarations across files
    let mut accumulated_decls: HashMap<String, ModuleDecls> = HashMap::new();
    
    for (i, file) in files.iter().take(5).enumerate() {
        println!("\n📁 Processing file {}: {}", i+1, file);
        
        let content = fs::read_to_string(format!("src/{}", file))?;
        let (header, _body) = split_header_body(&content);
        
        // Extract module dependencies and their types
        let module_deps = extract_module_dependencies(&header);
        for module in module_deps {
            let used_types = extract_used_types(&content, &module);
            
            // Create declarations for this module
            let mut module_decls = ModuleDecls::new();
            for type_name in used_types {
                let decl = format!("#[derive(Copy, Clone, Debug, PartialEq)]\npub struct {};", type_name);
                module_decls.types.insert(type_name.clone(), decl);
            }
            
            // Merge with accumulated declarations
            accumulated_decls.entry(module.clone())
                .or_insert_with(ModuleDecls::new)
                .merge(module_decls);
            
            println!("📦 Module '{}' now has {} types", module, accumulated_decls[&module].types.len());
        }
        
        // Generate current state
        let mut complete_decls = String::new();
        for (module_name, decls) in &accumulated_decls {
            complete_decls.push_str(&decls.generate_stub(module_name));
        }
        
        // Write progressive state
        fs::write(format!("progressive_decls_step_{}.rs", i+1), &complete_decls)?;
        println!("✅ Generated progressive_decls_step_{}.rs", i+1);
    }
    
    // Show final accumulated state
    println!("\n🎯 FINAL ACCUMULATED DECLARATIONS:");
    for (module_name, decls) in &accumulated_decls {
        println!("📦 Module '{}': {} types", module_name, decls.types.len());
        for type_name in decls.types.keys() {
            println!("   - {}", type_name);
        }
    }
    
    Ok(())
}

fn split_header_body(content: &str) -> (String, String) {
    let mut header = String::new();
    let mut body = String::new();
    let mut in_header = true;
    
    for line in content.lines() {
        let trimmed = line.trim();
        
        if (trimmed.starts_with("use ") || 
           trimmed.starts_with("pub use") ||
           trimmed.starts_with("#[cfg") ||
           trimmed.is_empty() ||
           trimmed.starts_with("//")) && in_header {
            header.push_str(line);
            header.push('\n');
        } else {
            in_header = false;
            body.push_str(line);
            body.push('\n');
        }
    }
    
    (header, body)
}

fn extract_module_dependencies(header: &str) -> Vec<String> {
    let mut modules = Vec::new();
    
    for line in header.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("pub use ") {
            if let Some(module) = trimmed.strip_prefix("pub use ") {
                if let Some(name) = module.split("::").next() {
                    if !name.starts_with("crate") && !name.starts_with("std") {
                        modules.push(name.to_string());
                    }
                }
            }
        }
    }
    
    modules.sort();
    modules.dedup();
    modules
}

fn extract_used_types(content: &str, module_name: &str) -> Vec<String> {
    let mut used_types = Vec::new();
    
    for line in content.lines() {
        let trimmed = line.trim();
        
        if trimmed.starts_with("pub use ") && trimmed.contains(&format!("{}::", module_name)) {
            if let Some(types_part) = trimmed.split('{').nth(1) {
                if let Some(types_str) = types_part.split('}').next() {
                    for type_name in types_str.split(',') {
                        let clean_name = type_name.trim();
                        if !clean_name.is_empty() {
                            used_types.push(clean_name.to_string());
                        }
                    }
                }
            }
        }
    }
    
    used_types
}
