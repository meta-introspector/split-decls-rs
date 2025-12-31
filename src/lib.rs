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

pub mod rustc_topological;
// pub mod rustc_test;  // Disabled temporarily
pub mod symbol_resolver;
pub mod dependency_extractor;

pub use symbol_resolver::*;
pub use dependency_extractor::*;
