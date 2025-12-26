// Generated from: ./src/macro_analyzer_parts/file_analyzer.rs
// Original file: ./src/macro_analyzer_parts/file_analyzer.rs
// Function: analyze_file_macros

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

#[decl_split_decls_rs_file_analyzer]
pub fn analyze_file_macros (file_path : & Path) -> Result < HashMap < String , Vec < Term > > > { let mut content = fs :: read_to_string (file_path) . with_context (| | format ! ("Failed to read file: {}" , file_path . display ())) ? ; if content . trim_end () . ends_with (". sig") { let trimmed_len = content . trim_end () . len () ; content . truncate (trimmed_len - ". sig" . len ()) ; } content = content . replace ("# [" , "#[") ; content = content . replace (" } " , "}\n") ; content = content . replace (" ] " , "]\n") ; let ast = parse_file (& content) . with_context (| | format ! ("Failed to parse Rust file: {}" , file_path . display ())) ? ; let mut macro_terms : HashMap < String , Vec < Term > > = HashMap :: new () ; for item in ast . items { if let syn :: Item :: Fn (func) = item { let is_pub = matches ! (func . vis , syn :: Visibility :: Public (_)) ; if func . sig . ident . to_string () . ends_with ("_impl") && is_pub { let mut collector = TermCollector :: default () ; collector . visit_item_fn (& func) ; macro_terms . insert (func . sig . ident . to_string () , collector . terms) ; } } } Ok (macro_terms) }