// Generated from: ./src/auto_workspace_generator.rs
// Original file: ./src/auto_workspace_generator.rs
// Function: generate_workspace_deps_from_project_root

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
pub fn generate_workspace_deps_from_project_root (project_root : & Path) -> Result < (Vec < String > , Vec < String >) > { println ! ("DEBUG: generate_workspace_deps_from_project_root called with: {}" , project_root . display ()) ; let mut workspace_deps = Vec :: new () ; let mut patch_entries = Vec :: new () ; let cargo_tomls = find_cargo_tomls (project_root) ? ; for cargo_path in cargo_tomls { if let Some (crate_info) = extract_crate_info (& cargo_path) ? { let relative_path = cargo_path . parent () . unwrap () . strip_prefix (project_root) ? . to_string_lossy () ; let dep_entry = format ! ("{} = {{ path = \"{}\" }}" , crate_info . name , relative_path) ; workspace_deps . push (dep_entry . clone ()) ; patch_entries . push (dep_entry) ; } } Ok ((workspace_deps , patch_entries)) }