// Generated from: ./src/bin/decl_histogram.rs
// Original file: ./src/bin/decl_histogram.rs
// Function: extract_decl_type

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

#[decl_split_decls_rs_decl_histogram]
fn extract_decl_type (filename : & str) -> Option < String > { if let Some (last_part) = filename . strip_suffix (".rs") { if let Some (type_name) = last_part . split ("_decls_") . nth (1) { if type_name . starts_with ("impl_for_") { return Some ("impl" . to_string ()) ; } else if filename . contains ("_struct_") || is_likely_struct (type_name) { return Some ("struct" . to_string ()) ; } else if filename . contains ("_enum_") || is_likely_enum (type_name) { return Some ("enum" . to_string ()) ; } else if filename . contains ("_fn_") || is_likely_function (type_name) { return Some ("function" . to_string ()) ; } else if filename . contains ("_const_") || type_name . chars () . all (| c | c . is_uppercase () || c == '_') { return Some ("const" . to_string ()) ; } else if filename . contains ("_trait_") { return Some ("trait" . to_string ()) ; } else if filename . contains ("_type_") { return Some ("type" . to_string ()) ; } else if filename . contains ("_static_") { return Some ("static" . to_string ()) ; } else { return Some ("other" . to_string ()) ; } } } None }