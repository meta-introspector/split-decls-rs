use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::Result;
use syn::{Item, ItemFn, ItemStruct, ItemEnum, ItemConst, ItemStatic};

fn main() -> Result<()> {
    let output_dir = Path::new("output3");
    let mut secretome = HashMap::new();
    
    // Scan all wrapped crates for declarations
    for entry in fs::read_dir(output_dir)? {
        let entry = entry?;
        if entry.file_name().to_string_lossy().starts_with("wrapped-") {
            process_crate(&entry.path(), &mut secretome)?;
        }
    }
    
    // Generate emoji mappings
    let emoji_map = generate_emoji_mappings(&secretome);
    
    // Save secretome
    fs::write("rustc_secretome.json", serde_json::to_string_pretty(&emoji_map)?)?;
    println!("🧬 Generated rustc secretome with {} unique symbols", emoji_map.len());
    
    Ok(())
}

fn process_crate(crate_path: &Path, secretome: &mut HashMap<String, SymbolInfo>) -> Result<()> {
    let decls_dir = crate_path.join("src/decls");
    if !decls_dir.exists() { return Ok(()); }
    
    for entry in fs::read_dir(decls_dir)? {
        let entry = entry?;
        if entry.path().extension().map_or(false, |ext| ext == "rs") {
            let content = fs::read_to_string(entry.path())?;
            if let Ok(file) = syn::parse_file(&content) {
                extract_symbols(&file, secretome);
            }
        }
    }
    Ok(())
}

fn extract_symbols(file: &syn::File, secretome: &mut HashMap<String, SymbolInfo>) {
    for item in &file.items {
        match item {
            Item::Fn(func) => {
                let key = format!("fn::{}", func.sig.ident);
                secretome.insert(key, SymbolInfo::Function(func.sig.ident.to_string()));
            }
            Item::Struct(s) => {
                let key = format!("struct::{}", s.ident);
                secretome.insert(key, SymbolInfo::Type(s.ident.to_string()));
            }
            Item::Enum(e) => {
                let key = format!("enum::{}", e.ident);
                secretome.insert(key, SymbolInfo::Type(e.ident.to_string()));
            }
            Item::Const(c) => {
                let key = format!("const::{}", c.ident);
                secretome.insert(key, SymbolInfo::Constant(c.ident.to_string()));
            }
            _ => {}
        }
    }
}

fn generate_emoji_mappings(secretome: &HashMap<String, SymbolInfo>) -> HashMap<String, String> {
    let mut emoji_map = HashMap::new();
    let mut emoji_index = 0;
    
    let emojis = [
        "🔧", "⚙️", "🛠️", "🔩", "⚡", "🔥", "💎", "🌟", "✨", "🎯", "🚀", "💫", "🌈", "🎨", "🎭",
        "🎪", "🎨", "🎯", "🎲", "🎳", "🎮", "🎰", "🎱", "🎲", "🎳", "🎴", "🎵", "🎶", "🎷", "🎸",
        "🎹", "🎺", "🎻", "🎼", "🎽", "🎾", "🎿", "🏀", "🏁", "🏂", "🏃", "🏄", "🏅", "🏆", "🏇",
        "🏈", "🏉", "🏊", "🏋", "🏌", "🏍", "🏎", "🏏", "🏐", "🏑", "🏒", "🏓", "🏔", "🏕", "🏖",
    ];
    
    for (key, _) in secretome {
        let emoji = emojis[emoji_index % emojis.len()];
        emoji_map.insert(key.clone(), emoji.to_string());
        emoji_index += 1;
    }
    
    emoji_map
}

#[derive(Debug)]
enum SymbolInfo {
    Function(String),
    Type(String),
    Constant(String),
}
