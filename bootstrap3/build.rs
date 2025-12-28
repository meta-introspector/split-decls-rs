use std::fs;
use std::path::Path;
use std::io::Write;

fn main() {
    println!("cargo:rerun-if-changed=../../output2");
    
    let output2_path = "../output2/wrapped-split-decls-rs/src/decls";
    let mut generated_code = String::new();
    
    generated_code.push_str("// Auto-generated mkwrap! macro implementation\n");
    generated_code.push_str("#[macro_export]\n");
    generated_code.push_str("macro_rules! mkwrap {\n");
    generated_code.push_str("    () => {\n");
    
    println!("cargo:warning=Scanning directory: {}", output2_path);
    
    if let Ok(entries) = fs::read_dir(output2_path) {
        let mut count = 0;
        for entry in entries.flatten() {
            let category = entry.file_name().to_string_lossy().to_string();
            let category_path = entry.path();
            
            if let Ok(type_entries) = fs::read_dir(&category_path) {
                for type_entry in type_entries.flatten() {
                    let type_name = type_entry.file_name().to_string_lossy().to_string();
                    let type_path = type_entry.path();
                    
                    if let Ok(num_entries) = fs::read_dir(&type_path) {
                        for num_entry in num_entries.flatten() {
                            let num = num_entry.file_name().to_string_lossy().to_string();
                            let num_path = num_entry.path();
                            
                            if let Ok(file_entries) = fs::read_dir(&num_path) {
                                for file_entry in file_entries.flatten() {
                                    let file_name = file_entry.file_name().to_string_lossy().to_string();
                                    if file_name.ends_with(".rs") {
                                        let name = file_name.trim_end_matches(".rs");
                                        generated_code.push_str(&format!(
                                            "        mkdeclmod!(\"{}\", \"{}\", \"{}\", \"{}\");\n",
                                            category, type_name, num, name
                                        ));
                                        count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        println!("cargo:warning=Generated {} mkdeclmod calls", count);
    } else {
        println!("cargo:warning=Could not read directory: {}", output2_path);
    }
    
    generated_code.push_str("    };\n");
    generated_code.push_str("}\n");
    
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("mkwrap_generated.rs");
    let mut f = fs::File::create(&dest_path).unwrap();
    f.write_all(generated_code.as_bytes()).unwrap();
    
    println!("cargo:warning=Generated mkwrap! macro at {:?}", dest_path);
}
