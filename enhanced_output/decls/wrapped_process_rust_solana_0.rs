// Generated from: ./src/bin/process_rust_solana.rs
// Original file: ./src/bin/process_rust_solana.rs
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

#[decl_split_decls_rs_process_rust_solana]
fn main () -> anyhow :: Result < () > { println ! ("🦀 PROCESSING RUST/SOLANA SUBMODULES") ; println ! ("=====================================") ; let config = SplitDeclsConfig :: default () ; let mut total_decls = 0 ; let mut processed_crates = 0 ; let target_crates = vec ! ["../../submodules/solana-sdk" , "../../submodules/rust-native-tls" , "../../submodules/rust-ini" , "../../submodules/curl-rust" , "../../submodules/litrs" , "../../submodules/ref-cast" , "../../submodules/ar_archive_writer" ,] ; for crate_path_str in target_crates { let crate_path = PathBuf :: from (crate_path_str) ; let lib_rs = crate_path . join ("src/lib.rs") ; let cargo_toml = crate_path . join ("Cargo.toml") ; if lib_rs . exists () && cargo_toml . exists () { println ! ("\n📦 Processing: {}" , crate_path . file_name () . unwrap () . to_string_lossy ()) ; let paths = setup_crate_paths (& crate_path) ? ; eager_splitter :: eager_split_crate (& paths , & config) ? ; let decl_count = if paths . decls_output_dir . exists () { fs :: read_dir (& paths . decls_output_dir) ? . count () } else { 0 } ; println ! ("   ✅ Generated {} declaration files" , decl_count) ; total_decls += decl_count ; processed_crates += 1 ; } else { println ! ("   ⚠️  Skipped: {} (missing lib.rs or Cargo.toml)" , crate_path . file_name () . unwrap () . to_string_lossy ()) ; } } println ! ("\n📊 RUST/SOLANA SUBMODULES SUMMARY:") ; println ! ("• Processed crates: {}" , processed_crates) ; println ! ("• Total declarations: {}" , total_decls) ; Ok (()) }