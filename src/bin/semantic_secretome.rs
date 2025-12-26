use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct SemanticSecretome {
    functions: HashMap<String, String>,
    types: HashMap<String, String>,
    constants: HashMap<String, String>,
    modules: HashMap<String, String>,
    total_symbols: usize,
    semantic_groups: HashMap<String, Vec<String>>,
}

fn main() -> Result<()> {
    let output_dir = Path::new("output3");
    let mut secretome = SemanticSecretome {
        functions: HashMap::new(),
        types: HashMap::new(),
        constants: HashMap::new(),
        modules: HashMap::new(),
        total_symbols: 0,
        semantic_groups: HashMap::new(),
    };
    
    // Scan all wrapped crates for declarations
    for entry in fs::read_dir(output_dir)? {
        let entry = entry?;
        if entry.file_name().to_string_lossy().starts_with("wrapped-") {
            process_crate(&entry.path(), &mut secretome)?;
        }
    }
    
    // Generate semantic groupings
    generate_semantic_groups(&mut secretome);
    
    // Assign emojis based on semantic meaning
    assign_semantic_emojis(&mut secretome);
    
    // Save secretome
    fs::write("rustc_semantic_secretome.json", serde_json::to_string_pretty(&secretome)?)?;
    println!("🧬 Generated semantic rustc secretome:");
    println!("   🔧 Functions: {}", secretome.functions.len());
    println!("   📦 Types: {}", secretome.types.len());
    println!("   🔢 Constants: {}", secretome.constants.len());
    println!("   📁 Modules: {}", secretome.modules.len());
    println!("   🎯 Semantic groups: {}", secretome.semantic_groups.len());
    println!("   🌟 Total symbols: {}", secretome.total_symbols);
    
    Ok(())
}

fn process_crate(crate_path: &Path, secretome: &mut SemanticSecretome) -> Result<()> {
    let decls_dir = crate_path.join("src/decls");
    if !decls_dir.exists() { return Ok(()); }
    
    let crate_name = crate_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .strip_prefix("wrapped-")
        .unwrap_or("unknown");
    
    for entry in fs::read_dir(decls_dir)? {
        let entry = entry?;
        if entry.path().extension().map_or(false, |ext| ext == "rs") {
            let content = fs::read_to_string(entry.path())?;
            if let Ok(file) = syn::parse_file(&content) {
                extract_symbols(&file, secretome, crate_name);
            }
        }
    }
    Ok(())
}

fn extract_symbols(file: &syn::File, secretome: &mut SemanticSecretome, crate_name: &str) {
    for item in &file.items {
        match item {
            syn::Item::Fn(func) => {
                let key = format!("{}::{}", crate_name, func.sig.ident);
                secretome.functions.insert(key, String::new());
                secretome.total_symbols += 1;
            }
            syn::Item::Struct(s) => {
                let key = format!("{}::{}", crate_name, s.ident);
                secretome.types.insert(key, String::new());
                secretome.total_symbols += 1;
            }
            syn::Item::Enum(e) => {
                let key = format!("{}::{}", crate_name, e.ident);
                secretome.types.insert(key, String::new());
                secretome.total_symbols += 1;
            }
            syn::Item::Const(c) => {
                let key = format!("{}::{}", crate_name, c.ident);
                secretome.constants.insert(key, String::new());
                secretome.total_symbols += 1;
            }
            _ => {}
        }
    }
}

fn generate_semantic_groups(secretome: &mut SemanticSecretome) {
    // Group by semantic categories
    let mut compiler_core = Vec::new();
    let mut memory_mgmt = Vec::new();
    let mut io_network = Vec::new();
    let mut parsing_ast = Vec::new();
    let mut crypto_hash = Vec::new();
    let mut error_handling = Vec::new();
    
    for key in secretome.functions.keys().chain(secretome.types.keys()).chain(secretome.constants.keys()) {
        let lower_key = key.to_lowercase();
        
        if lower_key.contains("rustc") || lower_key.contains("compiler") || lower_key.contains("codegen") {
            compiler_core.push(key.clone());
        } else if lower_key.contains("alloc") || lower_key.contains("memory") || lower_key.contains("heap") {
            memory_mgmt.push(key.clone());
        } else if lower_key.contains("io") || lower_key.contains("net") || lower_key.contains("http") {
            io_network.push(key.clone());
        } else if lower_key.contains("parse") || lower_key.contains("ast") || lower_key.contains("syn") {
            parsing_ast.push(key.clone());
        } else if lower_key.contains("hash") || lower_key.contains("crypto") || lower_key.contains("sha") {
            crypto_hash.push(key.clone());
        } else if lower_key.contains("error") || lower_key.contains("result") || lower_key.contains("panic") {
            error_handling.push(key.clone());
        }
    }
    
    secretome.semantic_groups.insert("compiler_core".to_string(), compiler_core);
    secretome.semantic_groups.insert("memory_management".to_string(), memory_mgmt);
    secretome.semantic_groups.insert("io_network".to_string(), io_network);
    secretome.semantic_groups.insert("parsing_ast".to_string(), parsing_ast);
    secretome.semantic_groups.insert("crypto_hash".to_string(), crypto_hash);
    secretome.semantic_groups.insert("error_handling".to_string(), error_handling);
}

fn assign_semantic_emojis(secretome: &mut SemanticSecretome) {
    // Semantic emoji mappings
    let function_emojis = ["🔧", "⚙️", "🛠️", "🔩", "⚡", "🚀", "💫", "✨"];
    let type_emojis = ["📦", "🏗️", "🧱", "🎯", "💎", "🌟", "🎨", "🎭"];
    let constant_emojis = ["🔢", "📊", "📈", "📉", "🎲", "🎯", "💯", "🔥"];
    
    // Collect keys first to avoid borrowing issues
    let function_keys: Vec<String> = secretome.functions.keys().cloned().collect();
    let type_keys: Vec<String> = secretome.types.keys().cloned().collect();
    let constant_keys: Vec<String> = secretome.constants.keys().cloned().collect();
    
    // Assign function emojis
    for (i, key) in function_keys.iter().enumerate() {
        let emoji = function_emojis[i % function_emojis.len()];
        secretome.functions.insert(key.clone(), emoji.to_string());
    }
    
    // Assign type emojis
    for (i, key) in type_keys.iter().enumerate() {
        let emoji = type_emojis[i % type_emojis.len()];
        secretome.types.insert(key.clone(), emoji.to_string());
    }
    
    // Assign constant emojis
    for (i, key) in constant_keys.iter().enumerate() {
        let emoji = constant_emojis[i % constant_emojis.len()];
        secretome.constants.insert(key.clone(), emoji.to_string());
    }
}
