use std::path::Path;
use std::fs;

pub fn process_crate(crate_path: &Path, output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Processing crate: {}", crate_path.display());
    
    // Find lib.rs or main.rs
    let entry_file = if crate_path.join("src/lib.rs").exists() {
        crate_path.join("src/lib.rs")
    } else if crate_path.join("src/main.rs").exists() {
        crate_path.join("src/main.rs")
    } else {
        return Ok(());
    };
    
    let content = fs::read_to_string(&entry_file)?;
    
    let crate_name = crate_path.file_name()
        .unwrap()
        .to_string_lossy()
        .replace('-', "_");
    
    let out_dir = output_dir.join(format!("wrapped-{}", crate_name));
    fs::create_dir_all(&out_dir.join("src/decls"))?;
    
    // Simple macro wrapper for the entire crate content
    let macro_content = format!(
        "macro_rules! {} {{\n    () => {{\n        {}\n    }};\n}}\n\n{}!();",
        crate_name, content, crate_name
    );
    
    let file_path = out_dir.join("src/decls").join(format!("{}.rs", crate_name));
    fs::write(&file_path, macro_content)?;
    
    // Generate lib.rs
    let lib_content = "pub mod decls;\npub use decls::*;\n";
    fs::write(out_dir.join("src/lib.rs"), lib_content)?;
    
    // Generate Cargo.toml
    let cargo_content = format!(
        "[package]\nname = \"wrapped-{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        crate_name
    );
    fs::write(out_dir.join("Cargo.toml"), cargo_content)?;
    
    println!("✅ Processed crate");
    Ok(())
}

pub fn bootstrap_from_output2(output2_dir: &Path, output3_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Bootstrap3: Generating output3 from output2");
    
    fs::create_dir_all(output3_dir)?;
    
    for entry in fs::read_dir(output2_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let crate_path = entry.path();
            if let Err(e) = process_crate(&crate_path, output3_dir) {
                println!("⚠️  Skipped {}: {}", crate_path.display(), e);
            }
        }
    }
    
    println!("🎉 Bootstrap3 complete!");
    Ok(())
}
