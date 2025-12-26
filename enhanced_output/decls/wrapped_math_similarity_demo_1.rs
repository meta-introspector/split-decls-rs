// Generated from: ./src/bin/math_similarity_demo.rs
// Original file: ./src/bin/math_similarity_demo.rs
// Function: analyze_k71_structure

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

#[decl_split_decls_rs_math_similarity_demo]
fn analyze_k71_structure () -> RustCodeStructure { RustCodeStructure { name : "complex_trait.rs" . to_string () , complexity : 6.2 , depth : 3 , dependencies : 8 , abstraction_level : 6.2 / 3.0 , } }