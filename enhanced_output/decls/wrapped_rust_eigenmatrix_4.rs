// Generated from: ./src/bin/rust_eigenmatrix.rs
// Original file: ./src/bin/rust_eigenmatrix.rs
// Function: save_cached_matrix

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

#[decl_split_decls_rs_rust_eigenmatrix]
fn save_cached_matrix (cache_file : & str , matrix : & [Vec < f64 >] , decls : & [Declaration]) -> Result < () > { let cached = CachedMatrix { matrix : matrix . to_vec () , decl_names : decls . iter () . map (| d | d . name . clone ()) . collect () , timestamp : std :: time :: SystemTime :: now () . duration_since (std :: time :: UNIX_EPOCH) ? . as_secs () , } ; let json = serde_json :: to_string (& cached) ? ; fs :: write (cache_file , json) ? ; println ! ("💾 Matrix cached to {}" , cache_file) ; Ok (()) }