// Generated from: ./src/bin/perf2bench.rs
// Original file: ./src/bin/perf2bench.rs
// Function: generate_bench_macros

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

#[decl_split_decls_rs_perf2bench]
fn generate_bench_macros (functions : Vec < PerfFunction > , output2_path : & Path) -> Vec < BenchMacro > { functions . into_iter () . map (| func | { let wrap_path = find_matching_decl (& func . function , output2_path) ; BenchMacro { perf_id : func . function . clone () , percentage : func . percentage , wrap_path , } }) . collect () }