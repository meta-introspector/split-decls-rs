// Generated from: ./src/bin/perf2bench.rs
// Original file: ./src/bin/perf2bench.rs
// Function: parse_perf_functions

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
fn parse_perf_functions (perf_file : & str) -> Result < Vec < PerfFunction > > { let content = fs :: read_to_string (perf_file) ? ; let mut functions = Vec :: new () ; for line in content . lines () { let parts : Vec < & str > = line . trim () . split_whitespace () . collect () ; if parts . len () >= 4 && parts [0] . ends_with ('%') { if let Ok (percentage) = parts [0] . trim_end_matches ('%') . parse :: < f64 > () { let binary = parts [1] . to_string () ; let function = parts [3 ..] . join (" ") ; functions . push (PerfFunction { percentage , binary , function , }) ; } } } Ok (functions) }