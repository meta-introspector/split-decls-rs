// Generated from: ./src/bin/self-discover-ast.rs
// Original file: ./src/bin/self-discover-ast.rs
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

#[decl_split_decls_rs_self-discover-ast]
fn main () -> anyhow :: Result < () > { println ! ("🎯 SELF-DISCOVERING AST GENERATOR") ; println ! ("Querying codebase to extract live syn types and generate visitor code\n") ; let mut discovery = SynTypeDiscovery :: new () ; println ! ("🔍 DISCOVERING TYPES FROM SYN SOURCE...") ; discovery . discover_from_syn_source () ? ; discovery . report_discovery () ; println ! ("\n🔧 GENERATING VISITOR MACRO...") ; let visitor_macro = discovery . generate_visitor_macro () ; fs :: write ("generated_visitor_macro.rs" , & visitor_macro) ? ; println ! ("  Saved to: generated_visitor_macro.rs") ; println ! ("\n✅ GENERATING QA ASSERTIONS...") ; let qa_assertions = discovery . generate_qa_assertions () ; fs :: write ("generated_qa_assertions.rs" , & qa_assertions) ? ; println ! ("  Saved to: generated_qa_assertions.rs") ; println ! ("\n📊 GENERATING PROOF REPORT...") ; let proof_report = generate_proof_report (& discovery) ; fs :: write ("discovery_proof_report.md" , & proof_report) ? ; println ! ("  Saved to: discovery_proof_report.md") ; println ! ("\n🎉 SELF-DISCOVERY COMPLETE!") ; println ! ("Generated visitor code covers {} discovered types" , discovery . discovered_types . len ()) ; Ok (()) }