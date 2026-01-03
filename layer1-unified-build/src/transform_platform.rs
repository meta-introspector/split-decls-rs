use crate::track_transform;
use syn::{parse_file, Item, Attribute, Meta};
use quote::quote;

/// Remove platform-specific items (Windows, Mac, etc.)
pub fn remove_platform_specific(content: &str) -> String {
    track_transform!(content, "REMOVE_PLATFORM", "Remove platform-specific AST items", |content: &str| {
        match parse_file(content) {
            Ok(mut file) => {
                file.items.retain(|item| !is_platform_specific(item));
                quote!(#file).to_string()
            }
            Err(_) => content.to_string()
        }
    })
}

fn is_platform_specific(item: &Item) -> bool {
    let attrs = match item {
        Item::Fn(f) => &f.attrs,
        Item::Struct(s) => &s.attrs,
        Item::Enum(e) => &e.attrs,
        Item::Impl(i) => &i.attrs,
        Item::Mod(m) => &m.attrs,
        _ => return false,
    };

    attrs.iter().any(|attr| is_platform_attr(attr))
}

fn is_platform_attr(attr: &Attribute) -> bool {
    // Simple string-based check for platform attributes
    let attr_str = quote!(#attr).to_string();
    attr_str.contains("cfg") && (
        attr_str.contains("windows") ||
        attr_str.contains("macos") ||
        attr_str.contains("target_os")
    )
}
