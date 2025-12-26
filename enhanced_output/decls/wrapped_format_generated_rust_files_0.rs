// Generated from: ./src/format_generated_rust_files.rs
// Original file: ./src/format_generated_rust_files.rs
// Function: format_generated_rust_files

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

#[decl_split_decls_rs_format_generated_rust_files]
pub fn format_generated_rust_files (output_dir : & Path , verbose : bool) -> Result < () > { if verbose { println ! ("DEBUG: Formatting generated Rust files in {}" , output_dir . display ()) ; } for entry in WalkDir :: new (output_dir) . into_iter () . filter_map (| e | e . ok ()) { let path = entry . path () ; if path . is_file () && path . extension () . map_or (false , | ext | ext == "rs") { if verbose { println ! ("DEBUG: Running rustfmt on {}" , path . display ()) ; } let output = Command :: new ("rustfmt") . arg (path) . output () . context (format ! ("Failed to execute rustfmt on {}" , path . display ())) ? ; if ! output . status . success () { eprintln ! ("WARNING: rustfmt failed on {}:" , path . display ()) ; eprintln ! ("{}" , String :: from_utf8_lossy (& output . stderr)) ; } } } Ok (()) }