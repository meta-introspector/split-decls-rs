use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Generate header (dependencies/imports) and body (declarations) for a processed file
pub fn generate_header_and_body(file_path: &str) -> Result<(String, String)> {
    let content = fs::read_to_string(file_path)?;
    
    let mut header = String::new();
    let mut body = String::new();
    let mut in_header = true;
    
    for line in content.lines() {
        let trimmed = line.trim();
        
        // Header: imports, uses, extern crates, cfg attributes
        if trimmed.starts_with("use ") || 
           trimmed.starts_with("extern crate") ||
           trimmed.starts_with("#[cfg") ||
           trimmed.starts_with("pub use") ||
           trimmed.is_empty() ||
           trimmed.starts_with("//") {
            if in_header {
                header.push_str(line);
                header.push('\n');
            }
        } else {
            // First non-header line switches to body
            in_header = false;
            body.push_str(line);
            body.push('\n');
        }
    }
    
    Ok((header, body))
}

/// Extract module dependencies from header
pub fn extract_dependencies(header: &str) -> Vec<String> {
    let mut deps = Vec::new();
    
    for line in header.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("pub use ") || trimmed.starts_with("use ") {
            if let Some(module) = extract_module_from_use(trimmed) {
                deps.push(module);
            }
        }
    }
    
    deps
}

fn extract_module_from_use(use_line: &str) -> Option<String> {
    // Extract first part of use statement
    let parts: Vec<&str> = use_line.split_whitespace().collect();
    if parts.len() >= 2 {
        let path = parts[1].trim_end_matches(';');
        let module = path.split("::").next()?;
        if !module.starts_with("crate") && !module.starts_with("std") {
            return Some(module.to_string());
        }
    }
    None
}

fn main() -> Result<()> {
    let file = "src/processed_rustc_abi_rustc_abi_src_callconv.rs";
    let (header, body) = generate_header_and_body(file)?;
    let deps = extract_dependencies(&header);
    
    println!("📋 HEADER:");
    println!("{}", header);
    println!("\n📦 DEPENDENCIES: {:?}", deps);
    println!("\n📄 BODY:");
    println!("{}", body.lines().take(10).collect::<Vec<_>>().join("\n"));
    
    Ok(())
}
