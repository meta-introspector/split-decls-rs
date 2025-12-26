// Generated from: ./src/bin/complexity_analyzer.rs
// Original file: ./src/bin/complexity_analyzer.rs
// Function: count_type_complexity

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

#[decl_split_decls_rs_complexity_analyzer]
fn count_type_complexity (ty : & syn :: Type) -> u8 { match ty { syn :: Type :: Path (path) => { let segments = & path . path . segments ; let base_complexity = if segments . len () > 2 { 2 } else { 1 } ; let generic_complexity = segments . iter () . map (| seg | match & seg . arguments { syn :: PathArguments :: AngleBracketed (args) => args . args . len () as u8 , _ => 0 , }) . sum :: < u8 > () ; base_complexity + generic_complexity } , syn :: Type :: Reference (_) => 1 , syn :: Type :: Tuple (tuple) => tuple . elems . len () as u8 , _ => 1 , } }