// Generated from: ./src/rustfmt_utils.rs
// Original file: ./src/rustfmt_utils.rs
// Function: format_rust_file

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

#[decl_split_decls_rs_rustfmt_utils]
pub fn format_rust_file (content : & str , path : & Path) -> Result < String > { let syntax_tree : syn :: File = syn :: parse_str (content) . context (format ! ("Failed to parse file: {}" , path . display ())) ? ; let formatted = catch_unwind (move | | { prettyplease :: unparse (& syntax_tree) }) . map_err (| e | { let panic_msg = if let Some (s) = e . downcast_ref :: < String > () { s . clone () } else if let Some (s) = e . downcast_ref :: < & str > () { s . to_string () } else { "An unknown panic occurred during prettyplease::unparse" . to_string () } ; anyhow :: anyhow ! ("prettyplease::unparse panicked for file {}: {}" , path . display () , panic_msg) }) ? ; Ok (formatted) }