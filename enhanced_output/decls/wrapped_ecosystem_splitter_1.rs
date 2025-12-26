// Generated from: ./src/bin/ecosystem_splitter.rs
// Original file: ./src/bin/ecosystem_splitter.rs
// Function: count_crates

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

#[decl_split_decls_rs_ecosystem_splitter]
fn count_crates (path : & PathBuf) -> Result < usize > { let mut count = 0 ; let mut dirs_scanned = 0 ; println ! ("🔍 Scanning: {}" , path . display ()) ; if path . join ("Cargo.toml") . exists () && path . join ("src/lib.rs") . exists () { count += 1 ; println ! ("📦 Found crate: {}" , path . display ()) ; } if let Ok (entries) = fs :: read_dir (path) { for entry in entries { if let Ok (entry) = entry { let entry_path = entry . path () ; if entry_path . is_dir () && ! entry_path . is_symlink () { let name = entry_path . file_name () . unwrap () . to_string_lossy () ; if ! name . starts_with ('.') && name != "target" && name != "node_modules" { dirs_scanned += 1 ; println ! ("📁 Entering: {}" , entry_path . display ()) ; count += count_crates (& entry_path) ? ; } } } } } Ok (count) }