#![feature(associated_type_bounds)]
#![feature(associated_type_defaults)]

use rustc_ast::mut_visit::MutVisitor;
use rustc_session::Session;
use rustc_feature::Features;

// Macro definitions
pub mod ext_ctxt_def;
pub mod ext_ctxt_generic;
pub mod expand_context_generic;
pub mod expand_context_type_def;
pub mod ref_a_mut_expand_context_b_drt_type_def;
pub mod derive_resolution_generic;
pub mod t_define_expansion_context_types_exts;
pub mod lint_store_expand_dyn_def;

mod ast_fragment;
pub mod config;
pub mod mac_result;

mod placeholders;
mod annotatable;
pub mod add_semicolon;
pub mod invocation_collector_node;
pub mod dummy_ast_node;
pub mod strip_unconfigured;
pub mod invocation_data;
pub mod base_expansion_context;
pub mod cfg_false_reporter;
pub mod prelude;
pub mod expanded_nodes; // New module
pub mod ast_traits; // New module
pub mod resolver_traits;
pub mod syntax_extension_trait;
pub mod context;
pub mod struct_macrostat;
pub mod errors;
pub mod tracemacro;
pub mod recursion_limit_reached;
pub mod wrong_fragment_kind;
pub mod ty_aliases;


pub use ast_fragment::*;
pub use config::*;
pub use mac_result::*;
pub use invocation_data::{ModuleData, DirOwnership};
pub use placeholders::PlaceholderExpander;