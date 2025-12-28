use anyhow::Result;
use std::fs;
use std::path::Path;
use std::collections::{HashMap, HashSet};
use syn::{File, Item};
use quote::ToTokens;

#[derive(Debug, Clone)]
struct DeclInfo {
    name: String,
    imports: HashSet<String>,
    exports: HashSet<String>,
}

fn extract_imports(item: &Item) -> HashSet<String> {
    let mut imports = HashSet::new();
    let tokens = item.to_token_stream().to_string();
    
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

fn extract_exports(item: &Item) -> HashSet<String> {
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

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let crate_path = if args.len() > 1 { &args[1] } else { "." };
    
    println!("🔍 Splitting crate: {}", crate_path);
    
    let lib_rs = Path::new(crate_path).join("src/lib.rs");
    if !lib_rs.exists() {
        println!("❌ No src/lib.rs found");
        return Ok(());
    }

    let content = fs::read_to_string(&lib_rs)?;
    let parsed: File = syn::parse_file(&content)?;
    
    let output_dir = Path::new(crate_path).join("src/decls");
    fs::create_dir_all(&output_dir)?;
    
    let mut declarations = HashMap::new();
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
        
        let imports = extract_imports(item);
        let exports = extract_exports(item);
        
        declarations.insert(name.clone(), DeclInfo { name, imports, exports });
        count += 1;
    }
    
    // Second pass: write files with macro system
    count = 0;
    for item in parsed.items {
        let name = match &item {
            Item::Fn(f) => f.sig.ident.to_string(),
            Item::Struct(s) => s.ident.to_string(),
            Item::Enum(e) => e.ident.to_string(),
            Item::Trait(t) => t.ident.to_string(),
            Item::Impl(_) => format!("impl_{}", count),
            _ => continue,
        };
        
        // Find macro dependencies
        let mut macro_deps = Vec::new();
        if let Some(decl) = declarations.get(&name) {
            for import in &decl.imports {
                for (other_name, other_decl) in &declarations {
                    if other_name != &name && other_decl.exports.contains(import) {
                        macro_deps.push(format!("{}!()", other_name));
                    }
                }
            }
        }
        
        let mut content = String::new();
        
        if !macro_deps.is_empty() {
            content.push_str("macro_rules! deps {\n");
            content.push_str("    () => {\n");
            for dep in &macro_deps {
                content.push_str(&format!("        {};\n", dep));
            }
            content.push_str("    };\n");
            content.push_str("}\n\n");
        }
        
        content.push_str(&format!("macro_rules! {} {{\n", name));
        content.push_str("    () => {\n");
        if !macro_deps.is_empty() {
            content.push_str("        deps!();\n");
        }
        content.push_str(&format!("        {}\n", item.to_token_stream()));
        content.push_str("    };\n");
        content.push_str("}\n\n");
        content.push_str(&format!("{}!()", name));
        
        let file_path = output_dir.join(format!("{}.rs", name));
        fs::write(file_path, content)?;
        count += 1;
    }
    
    println!("✅ Split {} declarations with macro dependencies", count);
    Ok(())
}
    
    fs::rename(&lib_rs, Path::new(crate_path).join("src/lib_old.rs"))?;
    fs::write(&lib_rs, new_lib)?;
    
    println!("✅ Split {} items", count);
    Ok(())
}
