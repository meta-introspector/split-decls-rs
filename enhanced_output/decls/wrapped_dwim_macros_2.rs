// Generated from: ./src/dwim_macros.rs
// Original file: ./src/dwim_macros.rs
// Function: find_best_macro_match

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

#[decl_split_decls_rs_dwim_macros]
fn find_best_macro_match (intent : & DwimIntent , macros : & HashMap < String , MacroDefinition >) -> MacroMatch { let matches : Vec < _ > = macros . values () . filter (| m | semantic_match_score (intent , m) > 0.7) . collect () ; match matches . len () { 0 => MacroMatch :: None , 1 => MacroMatch :: Single (matches [0] . clone ()) , _ => MacroMatch :: Ambiguous (matches . into_iter () . cloned () . collect ()) , } }