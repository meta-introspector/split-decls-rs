// Generated from: ./src/bin/update_split_decls_config.rs
// Original file: ./src/bin/update_split_decls_config.rs
// Function: generate_preservation_proof

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

#[decl_split_decls_rs_update_split_decls_config]
fn generate_preservation_proof (config : & SplitDeclsConfig , cargo2nix_root : & Path) -> Result < String > { let mut proof = String :: new () ; proof . push_str ("# Crate Preservation Proof\n\n") ; proof . push_str (& format ! ("Generated: {}\n" , chrono :: Utc :: now () . format ("%Y-%m-%d %H:%M:%S UTC"))) ; proof . push_str (& format ! ("Total crates: {}\n\n" , config . wrapping . crates . len ())) ; proof . push_str ("## Preservation Verification\n\n") ; let mut preserved_count = 0 ; let mut missing_count = 0 ; for crate_name in & config . wrapping . crates { if let Some (path_str) = config . crate_path_overrides . get (crate_name) { let path = Path :: new (path_str) ; let exists = path . exists () ; let cargo_toml_exists = path . join ("Cargo.toml") . exists () ; if exists && cargo_toml_exists { preserved_count += 1 ; proof . push_str (& format ! ("✅ `{}` → `{}`\n" , crate_name , path_str)) ; } else { missing_count += 1 ; proof . push_str (& format ! ("❌ `{}` → `{}` (missing)\n" , crate_name , path_str)) ; } } else { missing_count += 1 ; proof . push_str (& format ! ("❌ `{}` → (no path mapping)\n" , crate_name)) ; } } proof . push_str (& format ! ("\n## Summary\n\n")) ; proof . push_str (& format ! ("- **Preserved**: {}\n" , preserved_count)) ; proof . push_str (& format ! ("- **Missing**: {}\n" , missing_count)) ; proof . push_str (& format ! ("- **Preservation Rate**: {:.1}%\n" , (preserved_count as f64 / config . wrapping . crates . len () as f64) * 100.0)) ; if missing_count == 0 { proof . push_str ("\n🎉 **ALL CRATES PRESERVED** - Ready for bootstrap!\n") ; } else { proof . push_str (& format ! ("\n⚠️  {} crates need attention before bootstrap\n" , missing_count)) ; } Ok (proof) }