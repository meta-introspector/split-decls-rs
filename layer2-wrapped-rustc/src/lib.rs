#![recursion_limit = "256"]
#![allow(internal_features)]
#![feature(rustc_private)]
#![feature(never_type)]
#![feature(rustc_attrs)]
#![feature(lang_items)]
#![feature(optimize_attribute)]
#![feature(allocator_api)]
#![feature(maybe_uninit_write_slice)]
#![feature(panic_can_unwind)]
#![feature(extend_one)]
#![feature(proc_macro_diagnostic)]
#![feature(proc_macro_span)]
#![feature(proc_macro_tracked_env)]
#![feature(track_path)]
#![feature(proc_macro_expand)]
#![feature(proc_macro_internals)]
#![feature(proc_macro_value)]
#![feature(allow_internal_unstable)]
#![feature(allow_internal_unsafe)]
#![feature(staged_api)]
#![cfg_attr(bootstrap, feature(rustc_allow_const_fn_unstable))]

pub mod ourprelude;
pub mod build_lib;
pub mod rustc_matrix;
pub mod monster_trinity;
pub mod conformal_proof;
pub mod galois_transform;
pub mod rust_lattice;
pub mod symbol_cache;
pub mod daemon;
pub mod function_store;
pub mod function_graduator;
pub mod ast_tracer;
pub mod hir_tracer;

// Minimal external crates
pub mod compiler_feedback;
pub mod self_improving_compiler;
extern crate tracing;
extern crate synstructure;
extern crate proc_macro;

pub mod transformations;
pub mod transformation_tracker;
pub mod rustc_topological;
pub mod symbol_resolver;
pub mod dependency_extractor;
pub mod macro_wrappers;
pub mod preprocessing;

// Minimal stubs
pub mod fx;
pub mod sync;
pub mod graph;
pub mod common;


// Include the complete rustc code (generated)
pub mod rustc_complete;
pub use rustc_complete::*;

// Prelude for processed files
pub mod prelude {
    pub use crate::*;
    
    // Missing traits that processed files expect
    pub trait NoArgsAttributeParser {}
    pub trait AttributeParser {}
    pub struct AttributeTemplate;
    pub struct ParsedAttr;
}
// Export all main routine wrappers
pub mod exports {
    include!("exports.rs");
}
pub mod rustc_compose;

use rustc_matrix::RustcMatrix;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    RustcMatrix::run().await
}
