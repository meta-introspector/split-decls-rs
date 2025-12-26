// Generated from: ./src/all_file_scanner.rs
// Original file: ./src/all_file_scanner.rs
// Function: scan_rust_files_recursive

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

#[decl_split_decls_rs_all_file_scanner]
fn scan_rust_files_recursive (dir : & Path , files : & mut Vec < (PathBuf , Vec < Item >) >) -> Result < () > { for entry in fs :: read_dir (dir) ? { let entry = entry ? ; let path = entry . path () ; if path . is_dir () { scan_rust_files_recursive (& path , files) ? ; } else if path . extension () . map_or (false , | ext | ext == "rs") { if let Ok (content) = fs :: read_to_string (& path) { if let Ok (parsed) = syn :: parse_file (& content) { let functions : Vec < Item > = parsed . items . into_iter () . filter (| item | matches ! (item , Item :: Fn (_))) . collect () ; if ! functions . is_empty () { files . push ((path , functions)) ; } } } } } Ok (()) }