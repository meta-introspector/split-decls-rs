// Generated from: ./src/bin/math_similarity_demo.rs
// Original file: ./src/bin/math_similarity_demo.rs
// Function: extract_lmfdb_data

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
fn extract_lmfdb_data () -> Vec < EllipticCurveData > { vec ! [EllipticCurveData { field : "3.3.621.1" . to_string () , conductor_norm : 621 , curves_count : 1454 , isogeny_classes : 534 , complexity_ratio : 1454.0 / 534.0 , } , EllipticCurveData { field : "3.3.625.1" . to_string () , conductor_norm : 625 , curves_count : 1118 , isogeny_classes : 538 , complexity_ratio : 1118.0 / 538.0 , } ,] }