// Generated from: ./src/dwim_macros.rs
// Original file: ./src/dwim_macros.rs
// Function: discover_available_macros

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
fn discover_available_macros () -> HashMap < String , MacroDefinition > { let mut macros = HashMap :: new () ; macros . insert ("mkbootstrap" . to_string () , MacroDefinition { name : "mkbootstrap" . to_string () , purpose : "Generate bootstrap infrastructure" . to_string () , confidence : 0.9 , }) ; macros . insert ("mkbuildrs" . to_string () , MacroDefinition { name : "mkbuildrs" . to_string () , purpose : "Generate self-contained build.rs" . to_string () , confidence : 0.8 , }) ; macros }