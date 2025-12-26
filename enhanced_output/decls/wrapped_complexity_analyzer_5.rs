// Generated from: ./src/bin/complexity_analyzer.rs
// Original file: ./src/bin/complexity_analyzer.rs
// Function: calculate_complexity_score

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
fn calculate_complexity_score (field_count : usize , variant_count : usize , nested_depth : u8) -> u8 { let base_score = match field_count { 0 ..= 2 => 1 , 3 ..= 4 => 2 , 5 ..= 6 => 3 , 7 ..= 8 => 4 , 9 ..= 10 => 5 , 11 ..= 12 => 6 , _ => 7 , } ; let variant_bonus = match variant_count { 0 ..= 2 => 0 , 3 ..= 5 => 1 , 6 ..= 10 => 2 , _ => 3 , } ; let depth_bonus = nested_depth . min (3) ; (base_score + variant_bonus + depth_bonus) . min (10) }