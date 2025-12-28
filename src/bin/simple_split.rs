use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use syn::{File, Item};
use quote::ToTokens;

#[derive(Debug, Clone)]
struct DeclInfo {
    name: String,
    imports: HashSet<String>,
    exports: HashSet<String>,
    item_tokens: String, // Store the actual item as tokens
}

fn process_file_recursively(
    file_path: &Path,
    crate_root: &Path,
    output_dir: &Path,
    declarations: &mut HashMap<String, DeclInfo>,
    count: &mut usize,
) -> Result<()> {
    println!("📖 Processing file: {}", file_path.display());
    
    let content = fs::read_to_string(file_path)?;
    let parsed: File = syn::parse_file(&content)?;
    
    // Process all items in this file
    for item in &parsed.items {
        let (name, item_type) = match &item {
            Item::Fn(f) => {
                println!("  📝 Found function: {}", f.sig.ident);
                (f.sig.ident.to_string(), "function")
            },
            Item::Struct(s) => {
                println!("  🏗️  Found struct: {}", s.ident);
                (s.ident.to_string(), "struct")
            },
            Item::Enum(e) => {
                println!("  🔢 Found enum: {}", e.ident);
                (e.ident.to_string(), "enum")
            },
            Item::Trait(t) => {
                println!("  🎭 Found trait: {}", t.ident);
                (t.ident.to_string(), "trait")
            },
            Item::Impl(_) => {
                let impl_name = format!("impl_{}", *count);
                println!("  🔧 Found impl: {}", impl_name);
                (impl_name, "impl")
            },
            Item::Type(t) => {
                println!("  📋 Found type alias: {}", t.ident);
                (t.ident.to_string(), "type")
            },
            Item::Const(c) => {
                println!("  🔒 Found const: {}", c.ident);
                (c.ident.to_string(), "const")
            },
            Item::Static(s) => {
                println!("  📌 Found static: {}", s.ident);
                (s.ident.to_string(), "static")
            },
            Item::Mod(m) => {
                println!("  📦 Found module: {}", m.ident);
                
                // Process module recursively
                if m.content.is_some() {
                    // Inline module - items are already in this file
                    println!("    → Inline module (already processed)");
                } else {
                    // External module - find and process the .rs file
                    let module_file = find_module_file(file_path, &m.ident.to_string())?;
                    if let Some(mod_path) = module_file {
                        println!("    → External module, processing: {}", mod_path.display());
                        process_file_recursively(&mod_path, crate_root, output_dir, declarations, count)?;
                    }
                }
                (m.ident.to_string(), "module")
            },
            Item::Use(_) => {
                println!("  📥 Found use statement (skipping)");
                continue;
            },
            Item::Macro(m) => {
                if let Some(ident) = &m.ident {
                    println!("  🪄 Found macro: {}", ident);
                    (ident.to_string(), "macro")
                } else {
                    let macro_name = format!("macro_{}", *count);
                    println!("  🪄 Found unnamed macro: {}", macro_name);
                    (macro_name, "macro")
                }
            },
            _ => {
                let other_name = format!("other_{}", *count);
                println!("  ❓ Found other item: {} (type: {:?})", other_name, std::mem::discriminant(item));
                (other_name, "other")
            },
        };
        
        let imports = extract_imports(item);
        let exports = extract_exports(item);
        let item_tokens = item.to_token_stream().to_string();
        
        println!("    → {} ({}): {} imports, {} exports", name, item_type, imports.len(), exports.len());
        
        declarations.insert(name.clone(), DeclInfo { name, imports, exports, item_tokens });
        *count += 1;
    }
    
    Ok(())
}

fn find_module_file(current_file: &Path, module_name: &str) -> Result<Option<PathBuf>> {
    let parent = current_file.parent().unwrap();
    
    // Try module_name.rs
    let mod_file = parent.join(format!("{}.rs", module_name));
    if mod_file.exists() {
        return Ok(Some(mod_file));
    }
    
    // Try module_name/mod.rs
    let mod_dir = parent.join(module_name).join("mod.rs");
    if mod_dir.exists() {
        return Ok(Some(mod_dir));
    }
    
    println!("    ⚠️  Module file not found for: {}", module_name);
    Ok(None)
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
    let mut crate_path = ".";
    let mut output_dir_base = "output";
    
    // Parse arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--output-dir" => {
                if i + 1 < args.len() {
                    output_dir_base = &args[i + 1];
                    i += 2;
                } else {
                    i += 1;
                }
            }
            _ => {
                crate_path = &args[i];
                i += 1;
            }
        }
    }
    
    println!("🔍 Splitting crate: {}", crate_path);
    
    let lib_rs = Path::new(crate_path).join("src/lib.rs");
    if !lib_rs.exists() {
        println!("❌ No src/lib.rs found at: {}", lib_rs.display());
        return Ok(());
    }

    println!("📖 Reading file: {}", lib_rs.display());
    let content = fs::read_to_string(&lib_rs)?;
    let parsed: File = syn::parse_file(&content)?;
    
    // Create output directory in specified location
    let crate_name = Path::new(crate_path).file_name().unwrap().to_str().unwrap();
    let output_dir = Path::new(output_dir_base).join(format!("wrapped-{}", crate_name)).join("src/decls");
    println!("📁 Creating output directory: {}", output_dir.display());
    fs::create_dir_all(&output_dir)?;
    
    let mut declarations = HashMap::new();
    let mut count = 0;
    
    // Recursively process all files starting from lib.rs or main.rs
    println!("🔍 Recursively processing all files:");
    process_file_recursively(&lib_rs, Path::new(crate_path), &output_dir, &mut declarations, &mut count)?;
    
    // Second pass: write files with macro system (using all collected declarations)
    println!("✍️  Second pass - writing declaration files:");
    let mut file_count = 0;
    
    for (name, decl_info) in &declarations {
        // Find macro dependencies
        let mut macro_deps = Vec::new();
        for import in &decl_info.imports {
            for (other_name, other_decl) in &declarations {
                if other_name != name && other_decl.exports.contains(import) {
                    macro_deps.push(format!("{}!()", other_name));
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
        content.push_str(&format!("        {}\n", decl_info.item_tokens));
        content.push_str("    };\n");
        content.push_str("}\n\n");
        content.push_str(&format!("{}!()", name));
        
        let file_path = output_dir.join(format!("{}.rs", name));
        println!("  💾 Writing declaration file: {}", file_path.display());
        fs::write(&file_path, content)?;
        file_count += 1;
    }
    
    println!("✅ Split {} declarations with macro dependencies", count);
    Ok(())
}

