#![recursion_limit = "256"]
#![allow(internal_features)]
#![feature(rustc_private)]

// Minimal external crates
extern crate tracing;

// Basic modules
pub mod rustc_topological;
pub mod symbol_resolver;
pub mod dependency_extractor;

// Minimal stubs
pub mod fx;
pub mod sync;
pub mod graph;
pub mod common;
pub mod test_rustc_complete_access;

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
