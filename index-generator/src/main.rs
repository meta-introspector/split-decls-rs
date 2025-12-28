use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("# Output2 Index");
    println!("# Generated index of all declarations in output2");
    println!();
    
    for entry in fs::read_dir("../output2")? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let crate_path = entry.path();
            let crate_name = crate_path.file_name().unwrap().to_string_lossy();
            
            println!("[{}]", crate_name);
            
            let decls_dir = crate_path.join("src/decls");
            if decls_dir.exists() {
                scan_decls(&decls_dir, &crate_name)?;
            }
            println!();
        }
    }
    
    Ok(())
}

fn scan_decls(dir: &Path, crate_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            let file_name = path.file_stem().unwrap().to_string_lossy();
            let relative_path = path.strip_prefix("../output2").unwrap_or(&path);
            println!("{} = \"{}\"", file_name, relative_path.display());
        } else if path.is_dir() {
            scan_decls(&path, crate_name)?;
        }
    }
    Ok(())
}
