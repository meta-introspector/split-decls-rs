use crate::track_transform;
use syn::{parse_file, Item, ItemFn};
use quote::quote;

/// Remove test code and test modules
pub fn remove_test_code(content: &str) -> String {
    track_transform!(content, "REMOVE_TESTS", "Remove test code and test modules", |content: &str| {
        match parse_file(content) {
            Ok(mut file) => {
                file.items.retain(|item| !is_test_item(item));
                quote!(#file).to_string()
            }
            Err(_) => content.to_string()
        }
    })
}

fn is_test_item(item: &Item) -> bool {
    match item {
        Item::Fn(f) => is_test_function(f),
        Item::Mod(m) => {
            // Remove test modules
            m.ident == "tests" || 
            m.attrs.iter().any(|attr| {
                let attr_str = quote!(#attr).to_string();
                attr_str.contains("cfg") && attr_str.contains("test")
            })
        }
        _ => false,
    }
}

fn is_test_function(f: &ItemFn) -> bool {
    f.attrs.iter().any(|attr| {
        let attr_str = quote!(#attr).to_string();
        attr_str.contains("test") ||
        attr_str.contains("bench") ||
        (attr_str.contains("cfg") && attr_str.contains("test"))
    })
}
