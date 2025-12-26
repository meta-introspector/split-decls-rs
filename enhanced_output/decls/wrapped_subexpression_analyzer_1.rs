// Generated from: ./src/bin/subexpression_analyzer.rs
// Original file: ./src/bin/subexpression_analyzer.rs
// Function: analyze_expressions_in_file

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

#[decl_split_decls_rs_subexpression_analyzer]
fn analyze_expressions_in_file (file_path : & Path) -> Result < HashMap < String , Vec < String > > > { let content = fs :: read_to_string (file_path) ? ; let syntax_tree : File = syn :: parse_file (& content) ? ; let mut visitor = ExpressionVisitor :: new (file_path . to_string_lossy () . to_string ()) ; for item in & syntax_tree . items { syn :: visit :: visit_item (& mut visitor , item) ; } Ok (visitor . expressions) }