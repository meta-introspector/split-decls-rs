use syn::{parse_file, Item, File};
use std::fs;
use quote::quote;

fn add_prelude_syn(content: &str) -> Result<String, syn::Error> {
    let mut file: File = parse_file(content)?;
    
    // Create the prelude use statement
    let prelude_use: Item = syn::parse_quote! {
        use split_decls_genesis::ourprelude::*;
    };
    
    // Insert at the beginning of items (after attributes and comments)
    file.items.insert(0, prelude_use);
    
    Ok(quote!(#file).to_string())
}

fn main() {
    let source = fs::read_to_string("/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/library/std/src/os/uefi/env.rs")
        .expect("Failed to read source file");
    
    println!("Original: {:?}", parse_file(&source).is_ok());
    
    match add_prelude_syn(&source) {
        Ok(with_prelude) => {
            println!("With prelude: {:?}", parse_file(&with_prelude).is_ok());
            if let Err(err) = parse_file(&with_prelude) {
                println!("Error: {}", err);
            } else {
                println!("SUCCESS: Fixed the parsing issue!");
            }
        }
        Err(e) => println!("Failed to add prelude: {}", e),
    }
}
