use crate::track_transform;
use syn::{parse_file, Item, Attribute};
use quote::quote;

/// Remove rustc-internal diagnostic attributes
pub fn remove_diagnostic_attributes(content: &str) -> String {
    track_transform!(content, "REMOVE_DIAGNOSTIC_ATTRS", "Remove rustc-internal diagnostic attributes", |content: &str| {
        match parse_file(content) {
            Ok(mut file) => {
                // Remove diagnostic attributes from all items
                for item in &mut file.items {
                    remove_diagnostic_attrs_from_item(item);
                }
                quote!(#file).to_string()
            }
            Err(_) => content.to_string()
        }
    })
}

fn remove_diagnostic_attrs_from_item(item: &mut Item) {
    let attrs = match item {
        Item::Struct(s) => &mut s.attrs,
        Item::Enum(e) => &mut e.attrs,
        Item::Fn(f) => &mut f.attrs,
        Item::Impl(i) => &mut i.attrs,
        Item::Trait(t) => &mut t.attrs,
        Item::Mod(m) => &mut m.attrs,
        Item::Type(t) => &mut t.attrs,
        Item::Const(c) => &mut c.attrs,
        Item::Static(s) => &mut s.attrs,
        _ => return,
    };

    attrs.retain(|attr| !is_diagnostic_attribute(attr));
}

fn is_diagnostic_attribute(attr: &Attribute) -> bool {
    let attr_str = quote!(#attr).to_string();
    
    // List of rustc-internal diagnostic attributes
    let diagnostic_attrs = [
        "note", "label", "primary_span", "suggestion", "help", "diag",
        "subdiagnostic", "multipart_suggestion", "suggestion_part"
    ];
    
    diagnostic_attrs.iter().any(|&diag_attr| {
        attr_str.contains(&format!("# [{}]", diag_attr)) ||
        attr_str.contains(&format!("# [{}(", diag_attr))
    })
}
