// Generated from: ./src/bin/submodule_decl_report.rs
// Original file: ./src/bin/submodule_decl_report.rs
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

#[decl_split_decls_rs_submodule_decl_report]
fn extract_decl_type (file_path : & Path) -> Option < String > { let filename = file_path . file_stem () ? . to_str () ? ; if filename . contains ("_struct_") { Some ("struct" . to_string ()) } else if filename . contains ("_enum_") { Some ("enum" . to_string ()) } else if filename . contains ("_impl_") { Some ("impl" . to_string ()) } else if filename . contains ("_fn_") { Some ("function" . to_string ()) } else if filename . contains ("_const_") { Some ("const" . to_string ()) } else if filename . contains ("_trait_") { Some ("trait" . to_string ()) } else if filename . contains ("_type_") { Some ("type" . to_string ()) } else if filename . contains ("_static_") { Some ("static" . to_string ()) } else { Some ("other" . to_string ()) } }