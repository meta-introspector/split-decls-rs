use std::process::Command;
use std::fs;

fn main() {
    println!("🔧 Error Analysis Build Script");
    
    // Run cargo build and capture errors
    let output = Command::new("cargo")
        .args(&["build", "--message-format=json"])
        .output()
        .expect("Failed to run cargo build");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Extract and count errors
    let errors: Vec<&str> = stderr.lines()
        .filter(|line| line.contains("error:"))
        .collect();
    
    println!("📊 Found {} compilation errors", errors.len());
    
    // Save errors to file for analysis
    fs::write("compilation_errors.txt", stderr.as_ref())
        .expect("Failed to write errors");
    
    // Extract AST IDs from errors for patch generation
    let mut ast_ids = Vec::new();
    for error in &errors {
        if let Some(start) = error.find("AST_") {
            if let Some(end) = error[start..].find("_") {
                let ast_id = &error[start..start+end];
                ast_ids.push(ast_id);
            }
        }
    }
    
    println!("🎯 Extracted {} AST IDs for patching", ast_ids.len());
    
    // Generate patch list
    let patch_list = ast_ids.join("\n");
    fs::write("ast_patches_needed.txt", patch_list)
        .expect("Failed to write patch list");
    
    println!("✅ Error analysis complete - check compilation_errors.txt");
}
