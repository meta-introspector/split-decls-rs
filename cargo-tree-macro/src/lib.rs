use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

// Re-export CrateInfo so users of the macro can use it
pub use crate::CrateInfo;

// Include the generated data directly from the OUT_DIR.
// This assumes the build.rs script has successfully generated this file.
include!(concat!(env!("OUT_DIR"), "/cargo_tree_data.rs"));

#[proc_macro]
pub fn cargo_tree_data(_input: TokenStream) -> TokenStream {
    // This macro will provide access to the CARGO_TREE_DATA constant.
    // It is primarily used to ensure the `include!` happens and the constant is in scope.
    quote! { CARGO_TREE_DATA }.into()
}

// Structs for parsing `cargo tree` output within build.rs
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)] // Added Clone for easier handling
pub struct CrateInfo {
    pub name: String,
    pub path: String, // Relative path from the main split-decls-rs crate root
}

// Helper function to get the data (optional, can be directly accessed via CARGO_TREE_DATA)
pub fn get_cargo_tree_data() -> &'static [CrateInfo] {
    &CARGO_TREE_DATA
}