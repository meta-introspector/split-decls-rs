use syn::parse_file;
use std::fs;

fn main() {
    let content = fs::read_to_string("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/rustc_errors/src/translation.rs").unwrap();
    match parse_file(&content) {
        Ok(_) => println!("✅ Parses fine"),
        Err(e) => println!("❌ Error: {:?}", e),
    }
}
