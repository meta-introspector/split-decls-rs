// Generated from: ./src/bin/perf2bench.rs
// Original file: ./src/bin/perf2bench.rs
// Function: find_matching_decl

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

#[decl_split_decls_rs_perf2bench]
fn find_matching_decl (function_name : & str , output2_path : & Path) -> Option < String > { let crate_name = function_name . split ("::") . next () ? ; let wrapped_dir = output2_path . join (format ! ("wrapped-{}" , crate_name)) ; if ! wrapped_dir . exists () { return None ; } let decls_dir = wrapped_dir . join ("src/decls") ; if let Ok (entries) = fs :: read_dir (& decls_dir) { for entry in entries . flatten () { let file_name_string = entry . file_name () . to_string_lossy () . to_string () ; if file_name_string . contains (& function_name . replace ("::" , "_")) { return Some (entry . path () . to_string_lossy () . to_string ()) ; } } } None }