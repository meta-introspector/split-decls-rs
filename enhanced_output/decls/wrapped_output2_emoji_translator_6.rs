// Generated from: ./src/bin/output2_emoji_translator.rs
// Original file: ./src/bin/output2_emoji_translator.rs
// Function: verify_round_trip

use proc_macro::TokenStream;
use quote::quote;
use syn::*;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use split_decls_types::SplitDeclsConfig;
pub use extracted_decl::*;
pub use process_crate::process_crate;
pub use process_crates_in_path::process_crates_in_path;
pub use generate_wrapped_workspace::generate_wrapped_workspace;
prelude!{}

#[decl_split_decls_rs_output2_emoji_translator]
fn verify_round_trip (original : & [String] , reconstructed : & [String]) -> bool { if original . len () != reconstructed . len () { return false ; } for file in original { if ! reconstructed . contains (file) { return false ; } } true }