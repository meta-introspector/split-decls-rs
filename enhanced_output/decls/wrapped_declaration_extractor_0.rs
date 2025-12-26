// Generated from: ./src/eager_splitter/declaration_extractor.rs
// Original file: ./src/eager_splitter/declaration_extractor.rs
// Function: calculate_ast_metrics

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

#[decl_split_decls_rs_declaration_extractor]
fn calculate_ast_metrics (item : & Item) -> (usize , usize) { let mut visitor = AstMetricVisitor { max_depth : 0 , current_depth : 0 , node_count : 0 , } ; visitor . visit_item (item) ; (visitor . max_depth , visitor . node_count) }