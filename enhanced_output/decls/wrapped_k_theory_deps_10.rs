// Generated from: ./src/bin/k_theory_deps.rs
// Original file: ./src/bin/k_theory_deps.rs
// Function: load_declarations_recursive

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

#[decl_split_decls_rs_k_theory_deps]
fn load_declarations_recursive (dir : & str , decls : & mut Vec < Declaration >) -> Result < () > { for entry in fs :: read_dir (dir) ? { let entry = entry ? ; let path = entry . path () ; if path . is_dir () { load_declarations_recursive (& path . to_string_lossy () , decls) ? ; } else if path . extension () . map_or (false , | ext | ext == "rs") { let content = fs :: read_to_string (& path) ? ; let name = path . file_name () . unwrap () . to_string_lossy () . to_string () ; decls . push (Declaration { name , content }) ; } } Ok (()) }