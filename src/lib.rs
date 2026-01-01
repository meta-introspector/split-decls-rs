#![recursion_limit = "256"]
#![allow(internal_features)]
#![feature(rustc_private)]
#![feature(never_type)]
#![feature(rustc_attrs)]
#![feature(lang_items)]
#![feature(optimize_attribute)]
#![feature(allocator_api)]

// Minimal external crates
extern crate tracing;
extern crate rustc_target;
extern crate rustc_fs_util;
extern crate synstructure;
extern crate proc_macro;

// Basic modules
pub mod rustc_topological;
pub mod symbol_resolver;
pub mod dependency_extractor;

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
