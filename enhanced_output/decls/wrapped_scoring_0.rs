// Generated from: ./src/macro_analyzer_parts/scoring.rs
// Original file: ./src/macro_analyzer_parts/scoring.rs
// Function: get_closest_prime_reciprocal

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

#[decl_split_decls_rs_scoring]
pub fn get_closest_prime_reciprocal (relative_frequency : f64) -> f64 { if relative_frequency == 0.0 { return 0.0 ; } if relative_frequency >= 1.0 { return 1.0 ; } let mut closest_score = 0.0 ; let mut min_diff = f64 :: MAX ; for & p in PRIMES { let score = 1.0 / (p as f64) ; let diff = (relative_frequency - score) . abs () ; if diff < min_diff { min_diff = diff ; closest_score = score ; } } closest_score }