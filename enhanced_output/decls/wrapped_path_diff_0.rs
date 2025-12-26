// Generated from: ./src/path_diff.rs
// Original file: ./src/path_diff.rs
// Function: path_diff

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

#[decl_split_decls_rs_path_diff]
pub fn path_diff (from : & Path , to : & Path) -> Option < PathBuf > { let from_components : Vec < _ > = from . components () . collect () ; let to_components : Vec < _ > = to . components () . collect () ; let mut common_len = 0 ; for (a , b) in from_components . iter () . zip (to_components . iter ()) { if a == b { common_len += 1 ; } else { break ; } } let mut diff = PathBuf :: new () ; for _ in from_components . iter () . skip (common_len) { diff . push ("..") ; } for component in to_components . iter () . skip (common_len) { diff . push (component) ; } if diff . components () . next () . is_none () { Some (PathBuf :: from (".")) } else { Some (diff) } }