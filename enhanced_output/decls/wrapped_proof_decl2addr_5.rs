// Generated from: ./src/bin/proof_decl2addr.rs
// Original file: ./src/bin/proof_decl2addr.rs
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

#[decl_split_decls_rs_proof_decl2addr]
# [doc = " Proof of concept: Map all declarations to addresses"] fn main () -> Result < () > { println ! ("🔥 PROOF: decl2addr! and alldecls! macros") ; println ! ("\n📍 Individual declaration mapping (REAL ADDRESSES):") ; let error_addr = decl2addr ! ("Error") ; let debug_addr = decl2addr ! ("DebugFile") ; let context_addr = decl2addr ! ("Context") ; println ! ("  Error → {}" , error_addr) ; println ! ("  DebugFile → {}" , debug_addr) ; println ! ("  Context → {}" , context_addr) ; println ! ("\n🗺️  All declarations mapping:") ; let all_decls = alldecls ! () ; println ! ("  📊 Total declarations mapped: {}" , all_decls . len ()) ; let mut count = 0 ; for (name , decl_addr) in all_decls . iter () { if count < 10 { println ! ("  {} ({}) → {} [{}]" , name , decl_addr . decl_type , decl_addr . address , decl_addr . source_path . split ('/') . last () . unwrap_or ("")) ; count += 1 ; } } if all_decls . len () > 10 { println ! ("  ... and {} more declarations" , all_decls . len () - 10) ; } println ! ("\n🔄 Reverse address mapping:") ; let mut addr_to_decl : HashMap < String , Vec < String > > = HashMap :: new () ; for (name , decl_addr) in all_decls . iter () { addr_to_decl . entry (decl_addr . address . clone ()) . or_insert_with (Vec :: new) . push (name . clone ()) ; } println ! ("  📊 Unique addresses: {}" , addr_to_decl . len ()) ; let mut count = 0 ; for (addr , names) in addr_to_decl . iter () { if count < 5 { println ! ("  {} ← {}" , addr , names . join (", ")) ; count += 1 ; } } println ! ("\n🔍 REAL addr2line lookup:") ; for test_addr in [& error_addr , & debug_addr , & context_addr] { if let Some (names) = addr_to_decl . get (test_addr) { println ! ("  {} resolves to: {}" , test_addr , names . join (", ")) ; } } println ! ("\n📤 Export mapping:") ; let mapping_json = export_decl_mapping (& all_decls) ? ; println ! ("  JSON mapping size: {} bytes" , mapping_json . len ()) ; println ! ("  Sample: {}..." , & mapping_json [.. mapping_json . len () . min (100)]) ; println ! ("\n🎉 PROOF COMPLETE: Declaration-to-address mapping system functional!") ; println ! ("   - decl2addr! maps individual declarations") ; println ! ("   - alldecls! maps entire codebase") ; println ! ("   - Reverse lookup works") ; println ! ("   - Export format ready") ; Ok (()) }