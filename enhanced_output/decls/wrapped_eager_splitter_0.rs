// Generated from: ./src/eager_splitter.rs
// Original file: ./src/eager_splitter.rs
// Function: find_all_rust_files

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

#[decl_split_decls_rs_eager_splitter]
# [doc = " Recursively finds all .rs files in src directory that cargo would build"] fn find_all_rust_files (src_dir : & std :: path :: Path) -> Result < Vec < std :: path :: PathBuf > > { let mut rust_files = Vec :: new () ; if ! src_dir . exists () { return Ok (rust_files) ; } for entry in std :: fs :: read_dir (src_dir) ? { let entry = entry ? ; let path = entry . path () ; if path . is_file () && path . extension () . map_or (false , | ext | ext == "rs") { rust_files . push (path) ; } else if path . is_dir () { rust_files . extend (find_all_rust_files (& path) ?) ; } } Ok (rust_files) }