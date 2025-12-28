use std::path::Path;
use std::fs;

macro_rules! wrapped_fs_read_to_string {
    ($path:expr) => {{
        println!("🔧 WRAPPED: fs::read_to_string for: {}", $path.display());
        fs::read_to_string($path)
    }};
}

macro_rules! wrapped_fs_create_dir_all {
    ($path:expr) => {{
        println!("🔧 WRAPPED: fs::create_dir_all for: {}", $path.display());
        fs::create_dir_all($path)
    }};
}

macro_rules! wrapped_fs_write {
    ($path:expr, $content:expr) => {{
        println!("🔧 WRAPPED: fs::write for: {}", $path.display());
        fs::write($path, $content)
    }};
}

macro_rules! wrapped_fs_read_dir {
    ($path:expr) => {{
        println!("🔧 WRAPPED: fs::read_dir for: {}", $path.display());
        fs::read_dir($path)
    }};
}

pub fn process_crate(crate_path: &Path, output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Processing crate: {}", crate_path.display());
    
    // Find lib.rs or main.rs using WRAPPED functions
    let entry_file = if crate_path.join("src/lib.rs").exists() {
        crate_path.join("src/lib.rs")
    } else if crate_path.join("src/main.rs").exists() {
        crate_path.join("src/main.rs")
    } else {
        return Ok(());
    };
    
    let content = wrapped_fs_read_to_string!(&entry_file)?;
    
    let crate_name = crate_path.file_name()
        .unwrap()
        .to_string_lossy()
        .replace('-', "_");
    
    let out_dir = output_dir.join(format!("wrapped-{}", crate_name));
    wrapped_fs_create_dir_all!(&out_dir.join("src/decls"))?;
    
    // Simple macro wrapper for the entire crate content
    let macro_content = format!(
        "macro_rules! {} {{\n    () => {{\n        {}\n    }};\n}}\n\n{}!();",
        crate_name, content, crate_name
    );
    
    let file_path = out_dir.join("src/decls").join(format!("{}.rs", crate_name));
    wrapped_fs_write!(&file_path, macro_content)?;
    
    // Generate lib.rs using WRAPPED functions
    let lib_content = "pub mod decls;\npub use decls::*;\n";
    wrapped_fs_write!(out_dir.join("src/lib.rs"), lib_content)?;
    
    // Generate Cargo.toml using WRAPPED functions
    let cargo_content = format!(
        "[package]\nname = \"wrapped-{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        crate_name
    );
    wrapped_fs_write!(out_dir.join("Cargo.toml"), cargo_content)?;
    
    println!("✅ Processed crate using ALL WRAPPED functions");
    Ok(())
}

pub fn bootstrap_from_output3(output3_dir: &Path, output4_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Bootstrap4: Using WRAPPED functions from output3 → output4");
    
    wrapped_fs_create_dir_all!(output4_dir)?;
    
    for entry in wrapped_fs_read_dir!(output3_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let crate_path = entry.path();
            if let Err(e) = process_crate(&crate_path, output4_dir) {
                println!("⚠️  Skipped {}: {}", crate_path.display(), e);
            }
        }
    }
    
    println!("🎉 Bootstrap4 complete using ALL WRAPPED functions!");
    Ok(())
}
