use crate::track_transform;
use syn::{parse_file, Item, ItemFn, Visibility};
use quote::quote;

/// Remove unused private items
pub fn remove_unused_code(content: &str) -> String {
    track_transform!(content, "REMOVE_UNUSED", "Remove unused private items", |content: &str| {
        match parse_file(content) {
            Ok(mut file) => {
                // Simple heuristic: remove private items that look unused
                file.items.retain(|item| !is_likely_unused(item));
                quote!(#file).to_string()
            }
            Err(_) => content.to_string()
        }
    })
}

fn is_likely_unused(item: &Item) -> bool {
    match item {
        Item::Fn(f) => is_unused_function(f),
        Item::Struct(s) => matches!(s.vis, Visibility::Inherited) && s.ident.to_string().starts_with("_"),
        Item::Enum(e) => matches!(e.vis, Visibility::Inherited) && e.ident.to_string().starts_with("_"),
        _ => false,
    }
}

fn is_unused_function(f: &ItemFn) -> bool {
    // Remove private functions that start with underscore or contain "unused"
    matches!(f.vis, Visibility::Inherited) && 
    (f.sig.ident.to_string().starts_with("_") || 
     f.sig.ident.to_string().contains("unused"))
}
