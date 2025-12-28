use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=output2/");
    
    let mut mod_declarations = String::new();
    
    // Find all .rs files in output2 decls recursively
    if let Ok(files) = find_rs_files("output2/wrapped-split-decls-rs/src/decls") {
        for file_path in files {
            // Convert path to module name
            let rel_path = file_path.strip_prefix("output2/wrapped-split-decls-rs/src/decls/").unwrap();
            let mod_name = rel_path.replace('/', "_").replace(".rs", "");
            
            mod_declarations.push_str(&format!(
                "#[path = \"../{}\"]\npub mod {};\npub use {}::*;\n\n",
                file_path, mod_name, mod_name
            ));
        }
    }
    
    fs::write("src/output2_modules.rs", mod_declarations).ok();
}

fn find_rs_files(dir: &str) -> Result<Vec<String>, std::io::Error> {
    let mut files = Vec::new();
    find_rs_files_recursive(Path::new(dir), &mut files)?;
    Ok(files)
}

fn find_rs_files_recursive(dir: &Path, files: &mut Vec<String>) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            find_rs_files_recursive(&path, files)?;
        } else if path.extension().map_or(false, |ext| ext == "rs") {
            files.push(path.to_string_lossy().to_string());
        }
    }
    Ok(())
}
