use lib_introspector_core::detailed_parse_error;
use std::fs;

fn main() {
    let content = fs::read_to_string("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/rustc_errors/src/translation.rs")
        .expect("Failed to read file");
    
    println!("{}", detailed_parse_error(&content));
}
