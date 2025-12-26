// Generated from: ./src/bin/report_aliases.rs
// Original file: ./src/bin/report_aliases.rs
// Function: main

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

#[decl_split_decls_rs_report_aliases]
fn main () -> Result < () > { let mut aliases = HashMap :: new () ; for entry in WalkDir :: new ("../..") . max_depth (3) { let entry = entry ? ; if entry . file_name () == "Cargo.toml" && entry . path () . to_string_lossy () . contains ("submodules") { let content = fs :: read_to_string (entry . path ()) ? ; let crate_name = entry . path () . parent () . unwrap () . file_name () . unwrap () . to_str () . unwrap () ; if crate_name == "addr2line" { println ! ("DEBUG: Checking addr2line Cargo.toml") ; } if let Ok (toml) = toml :: from_str :: < Value > (& content) { for section in ["dependencies" , "dev-dependencies" , "build-dependencies"] { if let Some (deps) = toml . get (section) . and_then (| v | v . as_table ()) { for (dep_name , dep_spec) in deps { if let Some (table) = dep_spec . as_table () { if let Some (package) = table . get ("package") . and_then (| v | v . as_str ()) { aliases . insert (dep_name . clone () , (package . to_string () , crate_name . to_string ())) ; println ! ("DEBUG: Found alias {} -> {} in {}" , dep_name , package , crate_name) ; } } } } } } } } println ! ("Found {} dependency aliases:" , aliases . len ()) ; for (alias , (real_name , crate_name)) in aliases { println ! ("  {} -> {} (in {})" , alias , real_name , crate_name) ; } Ok (()) }