#![recursion_limit = "256"]
#![allow(internal_features)]
#![feature(lang_items)]
#![feature(yeet_expr)]
#![feature(negative_impls)]
#![feature(box_patterns)]
#![feature(decl_macro)]
#![feature(never_type)]
#![feature(try_blocks)]
#![feature(trait_alias)]
#![feature(rustc_attrs)]
#![feature(core_io_borrowed_buf)]
#![feature(assert_matches)]
#![feature(if_let_guard)]
#![feature(stmt_expr_attributes)]
#![feature(macro_metavar_expr)]
#![feature(cfg_select)]
#![feature(test)]
#![feature(type_alias_impl_trait)]
#![feature(alloc_error_handler)]

pub mod rustc_topological;
// pub mod rustc_test;  // Disabled temporarily
pub mod symbol_resolver;
pub mod dependency_extractor;

// Explicit modules (take precedence over generated stubs)
// Removed conflicting modules: rustc_index_macros, rustc_data_structures, rustc_infer
pub mod rustc_index;  // Restored - needed for imports
pub mod rustc_serialize;  // Restored - needed for imports
pub mod fx;
pub mod sync;
pub mod stable_hasher;
pub mod graph;
pub mod source_map;
pub mod ty;
pub mod rustc_abi;
pub mod common;
pub mod test_rustc_complete_access;
pub mod test_rustc_index;

// Include the complete rustc code (actual implementations)
pub mod rustc_complete;
pub use rustc_complete::*;
