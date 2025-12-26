// Generated from: ./src/auto_workspace_generator.rs
// Original file: ./src/auto_workspace_generator.rs
// Function: find_cargo_tomls

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

#[decl_split_decls_rs_auto_workspace_generator]
fn find_cargo_tomls (dir : & Path) -> Result < Vec < PathBuf > > { let mut cargo_tomls = Vec :: new () ; if dir . is_dir () { for entry in fs :: read_dir (dir) ? { let entry = entry ? ; let path = entry . path () ; if path . is_file () && path . file_name () == Some ("Cargo.toml" . as_ref ()) { cargo_tomls . push (path) ; } else if path . is_dir () && ! should_skip_dir (& path) { cargo_tomls . extend (find_cargo_tomls (& path) ?) ; } } } Ok (cargo_tomls) }