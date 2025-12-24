use proc_macro::TokenStream;
use quote::quote;
// use syn::{parse_macro_input, LitStr}; // No longer needed

// Re-export CrateInfo and get_cargo_tree_data from cargo-metadata-lib
pub use cargo_metadata_lib::{get_cargo_tree_data, CrateInfo};

// We no longer have a proc_macro here, as this crate will primarily re-export.
// The previous #[proc_macro] cargo_tree_data was trying to directly access
// CARGO_TREE_DATA which is now managed by cargo-metadata-lib.
// Users will directly call `cargo_metadata_lib::get_cargo_tree_data()`.