// Generated from: ./src/dwim_macros.rs
// Original file: ./src/dwim_macros.rs
// Function: iteratively_generate_macro

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
fn iteratively_generate_macro (intent : & DwimIntent) -> TokenStream { let state_url = share_generation_state (intent) ; quote ! { compile_error ! (concat ! ("No suitable macro found. Generated state at: " , # state_url , ". Please define appropriate macro or refine intent.")) ; } . into () }