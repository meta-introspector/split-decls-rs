#![recursion_limit = "512"]
#![allow(internal_features)]
#![allow(unused)]
#![allow(rustc::untranslatable_diagnostic)]
#![feature(rustc_private)]
#![feature(core_intrinsics)]
#![feature(decl_macro)]
#![feature(panic_backtrace_config)]
#![feature(panic_update_hook)]
#![feature(rustdoc_internals)]
#![feature(try_blocks)]

// Include our rustc_complete module with macros
#[macro_use]
#[path = "src/rustc_complete.rs"]
mod rustc_complete;

// Add extern crate declarations needed for rustc_driver
extern crate rustc_mir_dataflow;
extern crate rustc_infer;
extern crate rustc_parse_format;
extern crate rustc_hir_analysis;
extern crate rustc_ty_utils;
extern crate rustc_middle;
extern crate rustc_query_system;
extern crate rustc_hir_typeck;
extern crate rustc_hir;
extern crate rustc_attr_parsing;
extern crate rustc_builtin_macros;
extern crate rustc_interface;
extern crate rustc_mir_build;
extern crate rustc_symbol_mangling;
extern crate rustc_expand;
extern crate rustc_session;
extern crate rustc_mir_transform;
extern crate rustc_index;
extern crate rustc_next_trait_solver;
extern crate rustc_parse;
extern crate rustc_codegen_llvm;
extern crate rustc_lint;
extern crate rustc_public;
extern crate rustc_passes;
extern crate rustc_codegen_ssa;
extern crate rustc_trait_selection;
extern crate rustc_const_eval;
extern crate rustc_pattern_analysis;
extern crate rustc_span;
extern crate rustc_ast;
extern crate rustc_type_ir;
extern crate rustc_resolve;
extern crate rustc_borrowck;
extern crate rustc_data_structures;
extern crate rustc_serialize;
extern crate rustc_errors;
extern crate rustc_metadata;
extern crate rustc_transmute;
extern crate rustc_thread_pool;
extern crate rustc_incremental;
extern crate rustc_ast_ir;
extern crate rustc_traits;
extern crate rustc_macros;
extern crate rustc_ast_lowering;
extern crate rustc_driver_impl;
extern crate rustc_query_impl;
extern crate rustc_monomorphize;
extern crate rustc_ast_passes;
extern crate rustc_lint_defs;
extern crate rustc_arena;
extern crate rustc_abi;
extern crate rustc_sanitizers;
extern crate rustc_privacy;
extern crate rustc_hir_id;
extern crate rustc_baked_icu_data;
extern crate rustc_error_messages;
extern crate rustc_feature;
extern crate rustc_graphviz;
extern crate rustc_llvm;
extern crate rustc_public_bridge;
extern crate rustc_ast_pretty;
extern crate rustc_hir_pretty;
extern crate rustc_lexer;
extern crate rustc_index_macros;
extern crate rustc_log;
extern crate rustc_hashes;
extern crate rustc_fs_util;
extern crate rustc_fluent_macro;
extern crate rustc_type_ir_macros;
extern crate rustc_error_codes;
extern crate rustc_driver;

// Resolver macros to use processed rustc types
use_crate_rustc_mir_dataflow!();
use_crate_rustc_infer!();
use_crate_rustc_parse_format!();
use_crate_rustc_hir_analysis!();
use_crate_rustc_ty_utils!();
use_crate_rustc_middle!();
use_crate_rustc_query_system!();
use_crate_rustc_hir_typeck!();
use_crate_rustc_hir!();
use_crate_rustc_attr_parsing!();
use_crate_rustc_builtin_macros!();
use_crate_rustc_interface!();
use_crate_rustc_mir_build!();
use_crate_rustc_symbol_mangling!();
use_crate_rustc_codegen_gcc!();
use_crate_rustc_expand!();
use_crate_rustc_session!();
use_crate_rustc_mir_transform!();
use_crate_rustc_index!();
use_crate_rustc_next_trait_solver!();
use_crate_rustc_parse!();
use_crate_rustc_codegen_llvm!();
use_crate_rustc_lint!();
use_crate_rustc_public!();
use_crate_rustc_passes!();
use_crate_rustc_codegen_ssa!();
use_crate_rustc_trait_selection!();
use_crate_rustc_codegen_cranelift!();
use_crate_rustc_const_eval!();
use_crate_rustc_pattern_analysis!();
use_crate_rustc_span!();
use_crate_rustc_ast!();
use_crate_rustc_type_ir!();
use_crate_rustc_resolve!();
use_crate_rustc_borrowck!();
use_crate_rustc_data_structures!();
use_crate_rustc_serialize!();
use_crate_rustc_errors!();
use_crate_rustc_metadata!();
use_crate_rustc_transmute!();
use_crate_rustc_thread_pool!();
use_crate_rustc_incremental!();
use_crate_rustc_ast_ir!();
use_crate_rustc_traits!();
use_crate_rustc_macros!();
use_crate_rustc_ast_lowering!();
use_crate_rustc_driver_impl!();
use_crate_rustc_query_impl!();
use_crate_rustc_monomorphize!();
use_crate_rustc_ast_passes!();
use_crate_rustc_lint_defs!();
use_crate_rustc_arena!();
use_crate_rustc_abi!();
use_crate_rustc_sanitizers!();
use_crate_rustc_privacy!();
use_crate_rustc_hir_id!();
use_crate_rustc_baked_icu_data!();
use_crate_rustc_error_messages!();
use_crate_rustc_feature!();
use_crate_rustc_graphviz!();
use_crate_rustc_llvm!();
use_crate_rustc_public_bridge!();
use_crate_rustc_ast_pretty!();
use_crate_rustc_hir_pretty!();
use_crate_rustc_lexer!();
use_crate_rustc_index_macros!();
use_crate_rustc_log!();
use_crate_rustc_hashes!();
use_crate_rustc_fs_util!();
use_crate_rustc_fluent_macro!();
use_crate_rustc_type_ir_macros!();
use_crate_rustc_error_codes!();
use_crate_rustc_driver!();

fn main() {
    println!("🚀 Unified rustc wrapper with 73 crates loaded");
    rustc_driver_impl::main();
}
