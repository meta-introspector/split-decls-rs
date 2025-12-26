// Generated from: ./src/bin/layer_common_forms.rs
// Original file: ./src/bin/layer_common_forms.rs
// Function: truncate_string

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

#[decl_split_decls_rs_layer_common_forms]
fn truncate_string (s : & str , max_len : usize) -> String { if s . len () <= max_len { s . to_string () } else { format ! ("{}..." , & s [.. max_len - 3]) } }