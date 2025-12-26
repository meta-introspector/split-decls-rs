// Generated from: ./src/bin/output2_emoji_translator.rs
// Original file: ./src/bin/output2_emoji_translator.rs
// Function: scan_output2_files

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
fn scan_output2_files (path : & str) -> Result < Vec < String > > { let mut files = Vec :: new () ; if Path :: new (path) . exists () { for entry in walkdir :: WalkDir :: new (path) { let entry = entry ? ; if entry . file_type () . is_file () { if let Some (path_str) = entry . path () . to_str () { files . push (path_str . to_string ()) ; } } } } else { files = vec ! ["output2/Cargo.toml" . to_string () , "output2/src/lib.rs" . to_string () , "output2/src/main.rs" . to_string () , "output2/src/decls/mod.rs" . to_string () , "output2/wrapped_crates/rustc/Cargo.toml" . to_string () , "output2/wrapped_crates/rustc/src/lib.rs" . to_string () ,] ; } Ok (files) }