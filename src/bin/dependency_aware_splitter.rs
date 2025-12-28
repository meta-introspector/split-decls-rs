use anyhow::Result;
use std::fs;
use std::path::Path;
use std::collections::{HashMap, HashSet};
use syn::{File, Item, UseTree, Ident};
use quote::ToTokens;

#[derive(Debug, Clone)]
struct DeclInfo {
    name: String,
    imports: HashSet<String>,
    exports: HashSet<String>,
    file_path: String,
}

struct DependencyTracker {
    declarations: HashMap<String, DeclInfo>,
    global_imports: HashSet<String>,
}

impl DependencyTracker {
    fn new() -> Self {
        Self {
            declarations: HashMap::new(),
            global_imports: HashSet::new(),
        }
    }

    fn extract_imports(&self, item: &Item) -> HashSet<String> {
        let mut imports = HashSet::new();
        
        // Extract type references from the item's token stream
        let tokens = item.to_token_stream().to_string();
        
        // Simple heuristic: look for capitalized identifiers (types)
        for word in tokens.split_whitespace() {
            let clean = word.trim_matches(|c: char| !c.is_alphanumeric());
            if !clean.is_empty() && clean.chars().next().unwrap().is_uppercase() {
                if clean.len() > 1 && clean != "Self" {
                    imports.insert(clean.to_string());
                }
            }
        }
        
        imports
    }

    fn extract_exports(&self, item: &Item) -> HashSet<String> {
        let mut exports = HashSet::new();
        
        match item {
            Item::Fn(f) => { exports.insert(f.sig.ident.to_string()); }
            Item::Struct(s) => { exports.insert(s.ident.to_string()); }
            Item::Enum(e) => { exports.insert(e.ident.to_string()); }
            Item::Trait(t) => { exports.insert(t.ident.to_string()); }
            Item::Type(t) => { exports.insert(t.ident.to_string()); }
            _ => {}
        }
        
        exports
    }

    fn add_declaration(&mut self, name: String, item: &Item, file_path: String) {
        let imports = self.extract_imports(item);
        let exports = self.extract_exports(item);
        
        let decl_info = DeclInfo {
            name: name.clone(),
            imports,
            exports,
            file_path,
        };
        
        self.declarations.insert(name, decl_info);
    }

    fn generate_header(&self, decl_name: &str) -> String {
        if let Some(decl) = self.declarations.get(decl_name) {
            let mut header = String::new();
            header.push_str("// === MACRO DEPENDENCY SYSTEM ===\n");
            
            // Find internal dependencies as macro arguments
            let mut macro_deps = Vec::new();
            for import in &decl.imports {
                for (other_name, other_decl) in &self.declarations {
                    if other_name != decl_name && other_decl.exports.contains(import) {
                        macro_deps.push(format!("{}!()", other_name));
                    }
                }
            }
            
            if !macro_deps.is_empty() {
                header.push_str(&format!("// MACRO_DEPS: {}\n", macro_deps.join(", ")));
                header.push_str("macro_rules! deps {\n");
                header.push_str("    () => {\n");
                for dep in &macro_deps {
                    header.push_str(&format!("        {};\n", dep));
                }
                header.push_str("    };\n");
                header.push_str("}\n\n");
            }
            
            // Generate the declaration as a macro
            header.push_str(&format!("macro_rules! {} {{\n", decl_name));
            header.push_str("    () => {\n");
            if !macro_deps.is_empty() {
                header.push_str("        deps!();\n");
            }
            header.push_str("        // Declaration will be inserted here\n");
            header.push_str("    };\n");
            header.push_str("}\n\n");
            
            header
        } else {
            "".to_string()
        }
    }

    fn generate_dependency_map(&self) -> String {
        let mut output = String::new();
        output.push_str("// Auto-generated dependency map\n");
        output.push_str("use std::collections::HashMap;\n\n");
        output.push_str("#[macro_export]\n");
        output.push_str("macro_rules! dependency_map {\n");
        output.push_str("    () => {{\n");
        output.push_str("        {\n");
        output.push_str("            let mut deps: HashMap<&'static str, Vec<&'static str>> = HashMap::new();\n");
        
        for (decl_name, decl) in &self.declarations {
            let mut internal_deps = Vec::new();
            for import in &decl.imports {
                for (other_name, other_decl) in &self.declarations {
                    if other_name != decl_name && other_decl.exports.contains(import) {
                        internal_deps.push(format!("\"{}\"", other_name));
                    }
                }
            }
            
            if !internal_deps.is_empty() {
                output.push_str(&format!("            deps.insert(\"{}\", vec![{}]);\n", 
                                       decl_name, internal_deps.join(", ")));
            }
        }
        
        output.push_str("            deps\n");
        output.push_str("        }\n");
        output.push_str("    }};\n");
        output.push_str("}\n");
        
        output
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let crate_path = if args.len() > 1 { &args[1] } else { "." };
    
    println!("🔍 Splitting crate with dependency tracking: {}", crate_path);
    
    let lib_rs = Path::new(crate_path).join("src/lib.rs");
    if !lib_rs.exists() {
        println!("❌ No src/lib.rs found");
        return Ok(());
    }

    let content = fs::read_to_string(&lib_rs)?;
    let parsed: File = syn::parse_file(&content)?;
    
    let output_dir = Path::new(crate_path).join("src/decls");
    fs::create_dir_all(&output_dir)?;
    
    let mut tracker = DependencyTracker::new();
    let mut count = 0;
    
    // First pass: collect all declarations
    for item in &parsed.items {
        let name = match &item {
            Item::Fn(f) => f.sig.ident.to_string(),
            Item::Struct(s) => s.ident.to_string(),
            Item::Enum(e) => e.ident.to_string(),
            Item::Trait(t) => t.ident.to_string(),
            Item::Impl(_) => format!("impl_{}", count),
            _ => continue,
        };
        
        let file_path = format!("src/decls/{}.rs", name);
        tracker.add_declaration(name, item, file_path);
        count += 1;
    }
    
    // Second pass: write files with dependency headers
    for item in parsed.items {
        let name = match &item {
            Item::Fn(f) => f.sig.ident.to_string(),
            Item::Struct(s) => s.ident.to_string(),
            Item::Enum(e) => e.ident.to_string(),
            Item::Trait(t) => t.ident.to_string(),
            Item::Impl(_) => format!("impl_{}", count),
            _ => continue,
        };
        
        let header = tracker.generate_header(&name);
        let wrapped = format!(
            "{}// Actual declaration wrapped in macro:\n{}!()",
            header,
            name
        );
        
        let file_path = output_dir.join(format!("{}.rs", name));
        fs::write(file_path, wrapped)?;
    }
    
    // Generate dependency map file
    let dep_map = tracker.generate_dependency_map();
    fs::write("dependency_map.rs", dep_map)?;
    
    println!("✅ Split {} declarations with dependency tracking", tracker.declarations.len());
    println!("📊 Generated dependency_map.rs");
    
    Ok(())
}
