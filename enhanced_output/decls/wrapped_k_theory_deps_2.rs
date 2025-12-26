// Generated from: ./src/bin/k_theory_deps.rs
// Original file: ./src/bin/k_theory_deps.rs
// Function: extract_dependencies

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

#[decl_split_decls_rs_k_theory_deps]
fn extract_dependencies (content : & str) -> Vec < usize > { let patterns = ["use " , "impl " , "struct " , "enum " , "trait " , "fn " , "std::" , "crate::" , "super::" , "self::"] ; patterns . iter () . enumerate () . filter (| (_ , pattern) | content . contains (* pattern)) . map (| (i , _) | i % 100) . collect () }