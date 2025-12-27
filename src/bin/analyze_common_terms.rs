use anyhow::Result;
use std::collections::HashMap;
use std::process::Command;
use split_decls_rs::output2_macro_system::Output2MacroSystem;

/// Find common terms and check real addresses
fn main() -> Result<()> {
    println!("🔍 Finding common terms and checking real addresses");
    
    // Load all declarations
    let system = Output2MacroSystem::import_from_output2()?;
    println!("📦 Loaded {} declarations", system.macros.len());
    
    // Extract all terms from declaration names
    let mut term_counts: HashMap<String, Vec<String>> = HashMap::new();
    
    for (name, decl) in &system.macros {
        // Split name into terms (by underscore, camelCase, etc.)
        let terms = extract_terms(name);
        for term in terms {
            if term.len() >= 3 { // Only meaningful terms
                term_counts.entry(term.clone())
                    .or_insert_with(Vec::new)
                    .push(format!("{} ({})", name, decl.declaration_type));
            }
        }
    }
    
    // Find most common terms
    let mut common_terms: Vec<_> = term_counts.iter()
        .filter(|(_, items)| items.len() >= 5) // At least 5 occurrences
        .collect();
    common_terms.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    
    println!("\n🏆 Most common terms (top 20):");
    for (term, items) in common_terms.iter().take(20) {
        println!("  {} appears in {} declarations", term, items.len());
    }
    
    // Check real addresses for different declaration types
    println!("\n🎯 Checking real addresses by type:");
    
    let mut type_stats: HashMap<String, Vec<String>> = HashMap::new();
    let mut address_found = 0;
    let mut address_missing = 0;
    
    for (name, decl) in system.macros.iter().take(50) { // Sample first 50
        let addr_result = get_real_address(name);
        let has_addr = addr_result.starts_with("0x") && addr_result != "0x0";
        
        type_stats.entry(decl.declaration_type.clone())
            .or_insert_with(Vec::new)
            .push(format!("{}: {}", name, addr_result));
            
        if has_addr {
            address_found += 1;
        } else {
            address_missing += 1;
        }
    }
    
    println!("📊 Address statistics (sample of 50):");
    println!("  ✅ Found real addresses: {}", address_found);
    println!("  ❌ Missing addresses: {}", address_missing);
    
    // Show addresses by type
    for (decl_type, items) in &type_stats {
        println!("\n📋 {} declarations:", decl_type);
        for item in items.iter().take(5) {
            println!("    {}", item);
        }
        if items.len() > 5 {
            println!("    ... and {} more", items.len() - 5);
        }
    }
    
    // Test specific common terms
    println!("\n🔬 Testing common terms for addresses:");
    for (term, items) in common_terms.iter().take(10) {
        println!("\n  🏷️  Term: '{}'", term);
        
        // Test a few items with this term
        for item_info in items.iter().take(3) {
            let name = item_info.split(" (").next().unwrap_or("");
            let addr = get_real_address(name);
            let has_real_addr = addr.starts_with("0x") && addr != "0x0" && !addr.contains("hash");
            
            println!("    {} → {} {}", 
                name, 
                addr,
                if has_real_addr { "✅ REAL" } else { "❌ fallback" }
            );
        }
    }
    
    // Answer the key questions
    println!("\n❓ KEY QUESTIONS ANSWERED:");
    println!("1. Are all items in memory? {}", 
        if address_found > 0 { "Some are, some aren't" } else { "Most use fallback addresses" });
    println!("2. Do all items have addresses? YES - all get addresses (real or fallback)");
    println!("3. Even types? YES - types, structs, enums all get addresses");
    println!("4. Most common terms: {}", 
        common_terms.iter().take(3).map(|(t, _)| t.as_str()).collect::<Vec<_>>().join(", "));
    
    Ok(())
}

/// Extract terms from a declaration name
fn extract_terms(name: &str) -> Vec<String> {
    let mut terms = Vec::new();
    
    // Split by underscore
    for part in name.split('_') {
        if !part.is_empty() {
            terms.push(part.to_lowercase());
            
            // Also split camelCase within each part
            let camel_terms = split_camel_case(part);
            for term in camel_terms {
                if term.len() >= 2 {
                    terms.push(term.to_lowercase());
                }
            }
        }
    }
    
    terms.sort();
    terms.dedup();
    terms
}

/// Split camelCase into separate terms
fn split_camel_case(s: &str) -> Vec<String> {
    let mut terms = Vec::new();
    let mut current = String::new();
    
    for c in s.chars() {
        if c.is_uppercase() && !current.is_empty() {
            terms.push(current.clone());
            current.clear();
        }
        current.push(c);
    }
    
    if !current.is_empty() {
        terms.push(current);
    }
    
    terms
}

/// Get real memory address using multiple methods
fn get_real_address(symbol: &str) -> String {
    // Method 1: nm command
    if let Ok(addr) = get_nm_address(symbol) {
        return addr;
    }
    
    // Method 2: objdump
    if let Ok(addr) = get_objdump_address(symbol) {
        return addr;
    }
    
    // Method 3: Check if it's a Rust symbol in current binary
    if let Ok(addr) = get_rust_symbol_address(symbol) {
        return addr;
    }
    
    // Fallback: deterministic hash
    format!("0x{:x}(hash)", hash_symbol(symbol))
}

fn get_nm_address(symbol: &str) -> Result<String> {
    let output = #[syscall="exec"]
    Command::new("nm")
        .arg("-D")
        .arg("/proc/self/exe")
        .output()?;
        
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains(symbol) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 1 && parts[0] != "0000000000000000" {
                return Ok(format!("0x{}", parts[0]));
            }
        }
    }
    Err(anyhow::anyhow!("Not found"))
}

fn get_objdump_address(symbol: &str) -> Result<String> {
    let output = #[syscall="exec"]
    Command::new("objdump")
        .arg("-t")
        .arg("/proc/self/exe")
        .output()?;
        
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains(symbol) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 1 && parts[0] != "0000000000000000" {
                return Ok(format!("0x{}", parts[0]));
            }
        }
    }
    Err(anyhow::anyhow!("Not found"))
}

fn get_rust_symbol_address(symbol: &str) -> Result<String> {
    // Try to find mangled Rust symbols
    let mangled_patterns = vec![
        format!("_ZN*{}*", symbol),
        format!("{}*", symbol),
        format!("*{}*", symbol),
    ];
    
    let output = #[syscall="exec"]
    Command::new("nm")
        .arg("/proc/self/exe")
        .output()?;
        
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        for pattern in &mangled_patterns {
            if line.contains(symbol) && line.contains("T ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 1 && parts[0] != "0000000000000000" {
                    return Ok(format!("0x{}", parts[0]));
                }
            }
        }
    }
    Err(anyhow::anyhow!("Not found"))
}

fn hash_symbol(name: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    std::hash::Hasher::write(&mut hasher, name.as_bytes());
    std::hash::Hasher::finish(&hasher) & 0xFFFFFFFF
}
