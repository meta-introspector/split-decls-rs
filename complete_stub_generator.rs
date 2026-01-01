use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let file = "src/processed_rustc_abi_rustc_abi_src_callconv.rs";
    let content = fs::read_to_string(file)?;
    
    println!("🔧 Generating header and body for: {}", file);
    
    // 1. Split into header and body
    let (header, body) = split_header_body(&content);
    
    // 2. Extract all module dependencies from header
    let module_deps = extract_module_dependencies(&header);
    
    // 3. For each module, extract what types are used
    let mut all_stubs = String::new();
    for module in &module_deps {
        let used_types = extract_used_types(&content, module);
        let stub = generate_module_stub(module, &used_types);
        all_stubs.push_str(&stub);
        println!("📦 Module '{}' uses types: {:?}", module, used_types);
    }
    
    // 4. Generate skeleton from body
    let skeleton = generate_skeleton(&body);
    
    // 5. Create complete compilable file
    let complete_file = format!("{}\n{}\n{}", all_stubs, header, skeleton);
    
    fs::write("generated_stub.rs", &complete_file)?;
    println!("✅ Generated complete stub file: generated_stub.rs");
    
    // Test compilation
    println!("🧪 Testing compilation...");
    let output = std::process::Command::new("rustc")
        .args(&["--edition", "2021", "generated_stub.rs"])
        .output()?;
    
    if output.status.success() {
        println!("✅ Compilation successful!");
    } else {
        println!("❌ Compilation failed:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
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
           trimmed.starts_with("extern crate") ||
           trimmed.is_empty() ||
           trimmed.starts_with("//") ||
           trimmed.starts_with("///")) && in_header {
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
                    if !name.starts_with("crate") && !name.starts_with("std") && !name.starts_with("core") {
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

fn generate_module_stub(module_name: &str, used_types: &[String]) -> String {
    let mut stub = format!("pub mod {} {{\n", module_name);
    
    for type_name in used_types {
        stub.push_str(&format!("    #[derive(Copy, Clone, Debug, PartialEq)]\n"));
        stub.push_str(&format!("    pub struct {};\n\n", type_name));
    }
    
    stub.push_str("}\n\n");
    stub
}

fn generate_skeleton(body: &str) -> String {
    let mut skeleton = String::new();
    
    for line in body.lines() {
        let trimmed = line.trim();
        
        if trimmed.starts_with("pub enum ") {
            if let Some(name) = extract_name_after("pub enum ", trimmed) {
                skeleton.push_str(&format!("#[derive(Copy, Clone, Debug, PartialEq)]\npub enum {} {{ Stub }}\n\n", name));
            }
        } else if trimmed.starts_with("pub struct ") {
            if let Some(name) = extract_name_after("pub struct ", trimmed) {
                skeleton.push_str(&format!("#[derive(Copy, Clone, Debug, PartialEq)]\npub struct {};\n\n", name));
            }
        } else if trimmed.starts_with("pub fn ") {
            if let Some(name) = extract_fn_name(trimmed) {
                skeleton.push_str(&format!("pub fn {}() {{}}\n\n", name));
            }
        } else if trimmed.starts_with("impl ") && !trimmed.contains(" for ") {
            if let Some(name) = extract_name_after("impl ", trimmed) {
                skeleton.push_str(&format!("impl {} {{}}\n\n", name));
            }
        }
    }
    
    skeleton.push_str("fn main() {}\n");
    skeleton
}

fn extract_name_after(prefix: &str, line: &str) -> Option<String> {
    line.strip_prefix(prefix)?
        .split_whitespace()
        .next()?
        .split('<')
        .next()?
        .split('(')
        .next()?
        .trim_end_matches(';')
        .to_string()
        .into()
}

fn extract_fn_name(line: &str) -> Option<String> {
    line.strip_prefix("pub fn ")?
        .split('(')
        .next()
        .map(|s| s.to_string())
}
