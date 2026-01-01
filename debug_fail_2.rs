#![recursion_limit = "256"]
#![allow(internal_features)]
#![feature(lang_items)]
#![feature(test)]
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
#![feature(type_alias_impl_trait)]
#![feature(alloc_error_handler)]
#![feature(rustc_private)]

extern crate tracing;
extern crate rustc_ast;
extern crate rustc_middle;
extern crate rustc_span;
extern crate rustc_hir;
extern crate rustc_data_structures;
extern crate rustc_errors;
extern crate rustc_session;
extern crate smallvec;
extern crate thin_vec;

#[path = "src/dot_dot_rust_compiler_rustc_query_system_src_query_caches.rs"]
mod dot_dot_rust_compiler_rustc_query_system_src_query_caches;