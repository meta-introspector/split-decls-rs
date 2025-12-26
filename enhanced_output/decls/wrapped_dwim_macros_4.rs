// Generated from: ./src/dwim_macros.rs
// Original file: ./src/dwim_macros.rs
// Function: generate_macro_call

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
fn generate_macro_call (macro_def : & MacroDefinition , intent : & DwimIntent) -> TokenStream { match macro_def . name . as_str () { "mkbootstrap" => { quote ! { mkbootstrap ! { tools : ["wrap_single_crate" , "bootstrap"] , config : "split-decls-rs.toml" , workspace : true , recursive : true } } . into () } , "mkbuildrs" => { quote ! { mkbuildrs ! { dependencies : { syn = "2.0" , quote = "1.0" } , logic : dwim_build_logic ! () } } . into () } , _ => { quote ! { compile_error ! ("Unknown macro generation") ; } . into () } } }