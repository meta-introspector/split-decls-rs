#![recursion_limit = "256"]
#![allow(internal_features)]
#![feature(rustc_private)]
#![feature(never_type)]
#![feature(rustc_attrs)]
#![feature(lang_items)]
#![feature(optimize_attribute)]
#![feature(allocator_api)]
#![feature(try_blocks)]
#![feature(error_reporter)]

// Stub dependencies for rustc_errors translation.rs
pub mod rustc_error_messages {
    pub type FluentArgs = std::collections::HashMap<String, String>;
    pub struct LazyFallbackBundle;
}

pub mod tracing {
    pub fn debug(_: &str) {}
    pub fn trace(_: &str) {}
}

pub mod error {
    pub struct TranslateError;
    pub struct TranslateErrorKind;
}

pub mod snippet {
    pub struct Style;
}

pub struct DiagArg;
pub struct DiagMessage;
pub struct FluentBundle;

pub fn fallback_fluent_bundle(_: &str, _: Vec<String>) -> FluentBundle {
    FluentBundle
}

pub mod parsing;
pub mod transformations;
pub mod audit;

// Re-export main functions for convenience
pub use parsing::{test_parse, parse_rust_source, detailed_parse_error};
pub use transformations::{apply_transformation_by_name, add_prelude, fix_file_paths, process_content};
pub use audit::{ProcessingAudit, TransformationStep};
