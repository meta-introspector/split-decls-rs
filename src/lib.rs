#![recursion_limit = "256"]
#![allow(internal_features)]
#![feature(rustc_private)]
#![feature(never_type)]
#![feature(rustc_attrs)]
#![feature(lang_items)]
#![feature(optimize_attribute)]
#![feature(allocator_api)]

pub mod ourprelude;
pub mod build_lib;
pub mod rustc_matrix;
pub mod monster_trinity;
pub mod galois_transform;
pub mod rust_lattice;

// Minimal external crates
pub mod compiler_feedback;
pub mod self_improving_compiler;
extern crate tracing;
extern crate synstructure;
extern crate proc_macro;

// Basic modules
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
