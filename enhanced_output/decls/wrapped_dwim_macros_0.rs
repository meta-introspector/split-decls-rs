// Generated from: ./src/dwim_macros.rs
// Original file: ./src/dwim_macros.rs
// Function: dwim

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
# [doc = " DWIM (Do What I Mean) macro - deterministic macro discovery and generation"] # [doc = " "] # [doc = " Searches existing macros, finds best match, fails if ambiguous"] # [doc = " Iteratively generates macro calls, shares state via embedded ontology"] # [proc_macro] pub fn dwim (input : TokenStream) -> TokenStream { let intent = parse_macro_input ! (input as DwimIntent) ; let available_macros = discover_available_macros () ; match find_best_macro_match (& intent , & available_macros) { MacroMatch :: Single (macro_def) => { generate_macro_call (& macro_def , & intent) } , MacroMatch :: Ambiguous (matches) => { panic ! ("Ambiguous macro match: found {} candidates: {:?}" , matches . len () , matches) ; } , MacroMatch :: None => { iteratively_generate_macro (& intent) } } }