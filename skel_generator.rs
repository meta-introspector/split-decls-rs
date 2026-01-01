use std::fs;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Generate skeleton (stub declarations) from body content
pub fn generate_skeleton(body: &str) -> String {
    let mut skeleton = String::new();
    
    for line in body.lines() {
        let trimmed = line.trim();
        
        // Extract type declarations
        if trimmed.starts_with("pub enum ") {
            skeleton.push_str(&extract_enum_skeleton(line));
        } else if trimmed.starts_with("pub struct ") {
            skeleton.push_str(&extract_struct_skeleton(line));
        } else if trimmed.starts_with("pub fn ") {
            skeleton.push_str(&extract_fn_skeleton(line));
        } else if trimmed.starts_with("impl ") {
            skeleton.push_str(&extract_impl_skeleton(line));
        }
    }
    
    skeleton
}

fn extract_enum_skeleton(line: &str) -> String {
    if let Some(name) = extract_type_name(line, "pub enum ") {
        format!("#[derive(Copy, Clone, Debug, PartialEq)]\npub enum {} {{ Stub }}\n\n", name)
    } else {
        String::new()
    }
}

fn extract_struct_skeleton(line: &str) -> String {
    if let Some(name) = extract_type_name(line, "pub struct ") {
        format!("#[derive(Copy, Clone, Debug, PartialEq)]\npub struct {};\n\n", name)
    } else {
        String::new()
    }
}

fn extract_fn_skeleton(line: &str) -> String {
    if let Some(name) = extract_fn_name(line) {
        format!("pub fn {}() {{}}\n\n", name)
    } else {
        String::new()
    }
}

fn extract_impl_skeleton(line: &str) -> String {
    if let Some(name) = extract_impl_name(line) {
        format!("impl {} {{}}\n\n", name)
    } else {
        String::new()
    }
}

fn extract_type_name(line: &str, prefix: &str) -> Option<String> {
    line.strip_prefix(prefix)?
        .split_whitespace()
        .next()?
        .split('<')
        .next()
        .map(|s| s.to_string())
}

fn extract_fn_name(line: &str) -> Option<String> {
    line.strip_prefix("pub fn ")?
        .split('(')
        .next()
        .map(|s| s.to_string())
}

fn extract_impl_name(line: &str) -> Option<String> {
    line.strip_prefix("impl ")?
        .split_whitespace()
        .next()
        .map(|s| s.to_string())
}

/// Generate module stub from dependencies
pub fn generate_module_stubs(deps: &[String]) -> String {
    let mut stubs = String::new();
    
    for dep in deps {
        if !dep.starts_with("std") && !dep.starts_with("core") {
            stubs.push_str(&format!("pub mod {} {{\n    pub struct Stub;\n}}\n\n", dep));
        }
    }
    
    stubs
}

fn main() -> Result<()> {
    let file = "src/processed_rustc_abi_rustc_abi_src_callconv.rs";
    let content = fs::read_to_string(file)?;
    
    // Split into header and body
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
    
    // Extract dependencies
    let mut deps = Vec::new();
    for line in header.lines() {
        if line.trim().starts_with("pub use ") {
            if let Some(module) = line.split("::").next() {
                if let Some(name) = module.strip_prefix("pub use ") {
                    deps.push(name.to_string());
                }
            }
        }
    }
    
    // Generate skeleton
    let skeleton = generate_skeleton(&body);
    let module_stubs = generate_module_stubs(&deps);
    
    println!("📋 HEADER:\n{}", header);
    println!("📦 MODULE STUBS:\n{}", module_stubs);
    println!("🦴 SKELETON:\n{}", skeleton);
    
    // Write complete stub file
    let complete_stub = format!("{}\n{}\n{}", module_stubs, header, skeleton);
    fs::write("stub_file.rs", complete_stub)?;
    println!("✅ Generated stub_file.rs");
    
    Ok(())
}
