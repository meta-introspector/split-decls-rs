// Generated from: ./src/dwim_macros.rs
// Original file: ./src/dwim_macros.rs
// Function: share_generation_state

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
fn share_generation_state (intent : & DwimIntent) -> String { format ! ("https://dwim.split-decls.rs/state/{}" , intent . hash ()) }