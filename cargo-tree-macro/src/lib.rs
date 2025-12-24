use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

mod common_data; // Include the shared data definition

// Re-export CrateInfo so users of the macro can use it
use crate::common_data::CrateInfo;

// Include the generated data directly from the OUT_DIR.
// This assumes the build.rs script has successfully generated this file.
include!(concat!(env!("OUT_DIR"), "/cargo_tree_data.rs"));

#[proc_macro]
pub fn cargo_tree_data(_input: TokenStream) -> TokenStream {
    // This macro will provide access to the CARGO_TREE_DATA constant.
    // It is primarily used to ensure the `include!` happens and the constant is in scope.
    quote! { CARGO_TREE_DATA }.into()
}

// Helper function to get the data (optional, can be directly accessed via CARGO_TREE_DATA)
fn get_cargo_tree_data() -> &'static [CrateInfo] {
    &CARGO_TREE_DATA
}
