// Generated from: ./src/bin/extract_crate.rs
// Original file: ./src/bin/extract_crate.rs
// Function: copy_dir_recursive

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

#[decl_split_decls_rs_extract_crate]
# [doc = " Recursively copy directory"] fn copy_dir_recursive (src : & Path , dst : & Path) -> Result < () > { fs :: create_dir_all (dst) ? ; for entry in fs :: read_dir (src) ? { let entry = entry ? ; let src_path = entry . path () ; let dst_path = dst . join (entry . file_name ()) ; if src_path . is_dir () { copy_dir_recursive (& src_path , & dst_path) ? ; } else { fs :: copy (& src_path , & dst_path) ? ; } } Ok (()) }