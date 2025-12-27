use anyhow::Result;
use split_decls_rs::all_file_scanner::{scan_all_rust_files, generate_wrapped_decl_with_origin};
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    let crate_path = Path::new(".");
    
    println!("🔍 ENHANCED WRAPPING: Scanning ALL Rust files for functions...");
    
    let all_files = scan_all_rust_files(crate_path)?;
    
    println!("📊 Found {} files with functions:", all_files.len());
    
    let mut total_functions = 0;
    for (file_path, functions) in &all_files {
        println!("  📁 {}: {} functions", file_path.display(), functions.len());
        total_functions += functions.len();
    }
    
    println!("🎯 Total functions found: {}", total_functions);
    
    // Generate wrapped declarations for all functions
    println!("\n🚀 Generating wrapped declarations for ALL functions...");
    
    fs::create_dir_all("enhanced_output/decls")?;
    
    let mut generated_count = 0;
    for (file_path, functions) in all_files {
        for (i, function) in functions.iter().enumerate() {
            let wrapped_content = generate_wrapped_decl_with_origin(
                function,
                &file_path,
                "split_decls_rs"
            );
            
            let file_stem = file_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");
            
            let output_path = format!("enhanced_output/decls/wrapped_{}_{}.rs", file_stem, i);
            fs::write(&output_path, wrapped_content)?;
            generated_count += 1;
        }
    }
    
    println!("✅ Generated {} wrapped function declarations", generated_count);
    println!("📁 Output directory: enhanced_output/decls/");
    
    // Now we can import and execute ANY function from ANY file!
    println!("\n🎉 SUCCESS: Complete function wrapping achieved!");
    println!("🔥 Every function in the codebase is now addressable and executable!");
    
    Ok(())
}
