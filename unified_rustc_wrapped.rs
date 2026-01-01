#![recursion_limit = "512"]
#![allow(internal_features)]
#![allow(unused)]
#![feature(rustc_private)]
#![feature(core_intrinsics)]

// Add extern crate declarations needed for rustc_driver
extern crate rustc_driver;
extern crate rustc_driver_impl;
extern crate rustc_session;
extern crate rustc_middle;
extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_data_structures;
extern crate rustc_span;
extern crate rustc_errors;
extern crate rustc_interface;
extern crate rustc_codegen_ssa;
extern crate rustc_target;
extern crate rustc_metadata;
extern crate rustc_parse;
extern crate rustc_expand;
extern crate rustc_builtin_macros;
extern crate rustc_passes;
extern crate rustc_mir_build;
extern crate rustc_mir_transform;
extern crate rustc_mir_dataflow;
extern crate rustc_const_eval;
extern crate rustc_hir_analysis;
extern crate rustc_hir_typeck;
extern crate rustc_traits;
extern crate rustc_trait_selection;
extern crate rustc_infer;
extern crate rustc_borrowck;
extern crate rustc_privacy;
extern crate rustc_resolve;
extern crate rustc_lint;
extern crate rustc_serialize;
extern crate rustc_index;
extern crate rustc_macros;
extern crate rustc_query_system;
extern crate rustc_query_impl;
extern crate rustc_incremental;
extern crate rustc_symbol_mangling;
extern crate rustc_codegen_llvm;
extern crate rustc_llvm;
extern crate rustc_feature;
extern crate rustc_attr;
extern crate rustc_lexer;
extern crate rustc_arena;
extern crate rustc_type_ir;
extern crate rustc_fluent_macro;
extern crate rustc_log;
extern crate rustc_abi;
extern crate rustc_smir;
extern crate rustc_pattern_analysis;
extern crate rustc_next_trait_solver;

// Include macro wrappers
include!("src/macro_wrappers.rs");

// Include the actual processed rustc main file with macro wrappers
include!("processed_submodules_rust_compiler_rustc_src_main.rs");
