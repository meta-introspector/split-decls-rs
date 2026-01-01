use syn::{parse_file, Item, ItemMod, ItemUse, ItemFn, ItemStruct, ItemEnum, ItemTrait, ItemImpl};
use quote::quote;
use std::fs;

fn wrap_item(item: &Item) -> proc_macro2::TokenStream {
    match item {
        Item::Mod(item_mod) => {
            let mod_name = &item_mod.ident;
            let content = if let Some((_, items)) = &item_mod.content {
                let wrapped_items: Vec<_> = items.iter().map(wrap_item).collect();
                quote! { #(#wrapped_items)* }
            } else {
                quote! {}
            };
            quote! { mkmod!(#mod_name, { #content }) }
        }
        Item::Use(item_use) => {
            quote! { mkuse!(#item_use) }
        }
        Item::Fn(item_fn) => {
            quote! { mkitem!(mkfn!(#item_fn)) }
        }
        Item::Struct(item_struct) => {
            quote! { mkitem!(mkstruct!(#item_struct)) }
        }
        Item::Enum(item_enum) => {
            quote! { mkitem!(mkenum!(#item_enum)) }
        }
        Item::Trait(item_trait) => {
            quote! { mkitem!(mktrait!(#item_trait)) }
        }
        Item::Impl(item_impl) => {
            quote! { mkitem!(mkimpl!(#item_impl)) }
        }
        _ => quote! { mkitem!(#item) }
    }
}

fn process_file(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let ast = parse_file(&content)?;
    
    let wrapped_items: Vec<_> = ast.items.iter().map(wrap_item).collect();
    let result = quote! { #(#wrapped_items)* };
    
    Ok(result.to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Process all .rs files in submodules/rust/
    for entry in walkdir::WalkDir::new("submodules/rust/") {
        let entry = entry?;
        if entry.path().extension() == Some(std::ffi::OsStr::new("rs")) {
            let file_path = entry.path().to_str().unwrap();
            let wrapped_content = process_file(file_path)?;
            
            // Write to processed file
            let output_path = file_path.replace("submodules/rust/", "processed/");
            if let Some(parent) = std::path::Path::new(&output_path).parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&output_path, wrapped_content)?;
            println!("Processed: {} -> {}", file_path, output_path);
        }
    }
    
    Ok(())
}
