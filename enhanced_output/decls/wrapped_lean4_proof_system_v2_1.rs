// Generated from: ./src/bin/lean4_proof_system_v2.rs
// Original file: ./src/bin/lean4_proof_system_v2.rs
// Function: execute_lean4_proof

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

#[decl_split_decls_rs_lean4_proof_system_v2]
fn execute_lean4_proof () -> bool { use std :: process :: Command ; println ! ("\n⚡ STEP 3: LEAN4 EXECUTION") ; match Command :: new ("lean") . args (& ["--run" , "lean4_proof/Main.lean"]) . current_dir (".") . output () { Ok (output) => { if output . status . success () { println ! ("   ✅ Lean4 proof executed successfully!") ; if ! output . stdout . is_empty () { println ! ("   📤 Lean4 output:") ; println ! ("{}" , String :: from_utf8_lossy (& output . stdout)) ; } true } else { println ! ("   📝 Lean4 execution failed (proof files generated)") ; if ! output . stderr . is_empty () { println ! ("   ⚠️  Error: {}" , String :: from_utf8_lossy (& output . stderr)) ; } false } } Err (_) => { println ! ("   📝 Lean4 not available (proof files generated)") ; false } } }