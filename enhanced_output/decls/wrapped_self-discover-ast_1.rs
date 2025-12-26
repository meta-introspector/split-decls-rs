// Generated from: ./src/bin/self-discover-ast.rs
// Original file: ./src/bin/self-discover-ast.rs
// Function: generate_proof_report

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
fn generate_proof_report (discovery : & SynTypeDiscovery) -> String { let mut report = String :: from ("# Self-Discovering AST Generator Proof Report\n\n") ; report . push_str ("## Discovery Summary\n\n") ; report . push_str (& format ! ("- **Discovered Types**: {}\n" , discovery . discovered_types . len ())) ; report . push_str (& format ! ("- **Visit Methods**: {}\n" , discovery . visit_methods . len ())) ; report . push_str (& format ! ("- **Enum Variants**: {}\n" , discovery . enum_variants . len ())) ; report . push_str ("\n## Discovered Types\n\n") ; let mut types : Vec < _ > = discovery . discovered_types . iter () . collect () ; types . sort () ; for type_name in types { report . push_str (& format ! ("- `{}`\n" , type_name)) ; } report . push_str ("\n## Generated Visit Methods\n\n") ; let mut methods : Vec < _ > = discovery . visit_methods . iter () . collect () ; methods . sort () ; for method in methods { report . push_str (& format ! ("- `{}`\n" , method)) ; } report . push_str ("\n## QA Assertions\n\n") ; report . push_str ("The generated code includes assertions that verify:\n") ; report . push_str ("1. All expected core types are discovered\n") ; report . push_str ("2. Minimum type coverage is achieved\n") ; report . push_str ("3. Generated visitor methods match discovered types\n") ; report . push_str ("\n## Self-Application Proof\n\n") ; report . push_str ("This system demonstrates self-application by:\n") ; report . push_str ("1. **Querying its own codebase** to discover live types\n") ; report . push_str ("2. **Generating visitor code** based on actual findings\n") ; report . push_str ("3. **Creating QA tests** to verify completeness\n") ; report . push_str ("4. **Producing this proof report** as evidence\n") ; report }