// Generated from: ./src/bin/perf2bench.rs
// Original file: ./src/bin/perf2bench.rs
// Function: main

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
fn main () -> Result < () > { let perf_file = "user_functions.txt" ; let output2_path = Path :: new ("output2") ; let functions = parse_perf_functions (perf_file) ? ; let bench_macros = generate_bench_macros (functions , output2_path) ; println ! ("// Generated perf2bench report") ; println ! ("// Maps perf hotspots to output2 macro declarations\n") ; for macro_call in bench_macros { match macro_call . wrap_path { Some (path) => { println ! ("!perfreport !id(\"{}\") !stats({:.2}%) !wrap-macro(\"{}\")" , macro_call . perf_id , macro_call . percentage , path) ; } None => { println ! ("!perfreport !id(\"{}\") !stats({:.2}%) !wrap-macro(\"NOT_FOUND\")" , macro_call . perf_id , macro_call . percentage) ; } } } Ok (()) }