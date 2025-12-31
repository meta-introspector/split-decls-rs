//! Auto-generated rustc components in topological dependency order
//! Level 0: rustc::main::main (entry point)
//! Level 1: All compiler components

// LEVEL 0: Entry Point
// rustc::main::main - The compiler entry point
// include!("../rust/compiler/rustc/src/main.rs");

// LEVEL 1: Compiler Components (in dependency order)
// 1: rustc_span (2754 lines)
// include!("../rust/compiler/rustc_span/src/lib.rs");
// 2: rustc_const_eval (60 lines)
// include!("../rust/compiler/rustc_const_eval/src/lib.rs");
// 3: rustc_transmute (175 lines)
// include!("../rust/compiler/rustc_transmute/src/lib.rs");
// 4: rustc_ast_passes (18 lines)
// include!("../rust/compiler/rustc_ast_passes/src/lib.rs");
// 5: rustc_symbol_mangling (314 lines)
// include!("../rust/compiler/rustc_symbol_mangling/src/lib.rs");
// 6: rustc_privacy (1886 lines)
// include!("../rust/compiler/rustc_privacy/src/lib.rs");
// 7: rustc_parse (273 lines)
// include!("../rust/compiler/rustc_parse/src/lib.rs");
// 8: rustc_public_bridge (305 lines)
// include!("../rust/compiler/rustc_public_bridge/src/lib.rs");
// 9: proc_macro (1616 lines)
// include!("../rust/library/proc_macro/src/lib.rs");
// 10: panic_unwind (112 lines)
// include!("../rust/library/panic_unwind/src/lib.rs");
// 11: rustc_expand (34 lines)
// include!("../rust/compiler/rustc_expand/src/lib.rs");
// 12: rustc_hir (48 lines)
// include!("../rust/compiler/rustc_hir/src/lib.rs");
// 13: rustc_pattern_analysis (148 lines)
// include!("../rust/compiler/rustc_pattern_analysis/src/lib.rs");
// 14: rustc_next_trait_solver (18 lines)
// include!("../rust/compiler/rustc_next_trait_solver/src/lib.rs");
// 15: rustc_trait_selection (42 lines)
// include!("../rust/compiler/rustc_trait_selection/src/lib.rs");
// 16: rustc_codegen_cranelift (395 lines)
// include!("../rust/compiler/rustc_codegen_cranelift/src/lib.rs");
// 17: rustc_index (52 lines)
// include!("../rust/compiler/rustc_index/src/lib.rs");
// 18: rustc_resolve (2565 lines)
// include!("../rust/compiler/rustc_resolve/src/lib.rs");
// 19: rustc_error_messages (637 lines)
// include!("../rust/compiler/rustc_error_messages/src/lib.rs");
// 20: rustc_driver_impl (1634 lines)
// include!("../rust/compiler/rustc_driver_impl/src/lib.rs");
// 21: rustc_query_impl (241 lines)
// include!("../rust/compiler/rustc_query_impl/src/lib.rs");
// 22: alloc (245 lines)
// include!("../rust/library/alloc/src/lib.rs");
// 23: rustc_thread_pool (894 lines)
// include!("../rust/compiler/rustc_thread_pool/src/lib.rs");
// 24: core (400 lines)
// include!("../rust/library/core/src/lib.rs");
// 25: rustc_public (302 lines)
// include!("../rust/compiler/rustc_public/src/lib.rs");
// 26: rustc_passes (53 lines)
// include!("../rust/compiler/rustc_passes/src/lib.rs");
// 27: rustc_codegen_llvm (444 lines)
// include!("../rust/compiler/rustc_codegen_llvm/src/lib.rs");
// 28: rustc_ast_pretty (11 lines)
// include!("../rust/compiler/rustc_ast_pretty/src/lib.rs");
// 29: rustc_ast_lowering (2641 lines)
// include!("../rust/compiler/rustc_ast_lowering/src/lib.rs");
// 30: rustc_hir_typeck (548 lines)
// include!("../rust/compiler/rustc_hir_typeck/src/lib.rs");
// 31: rustc_ty_utils (54 lines)
// include!("../rust/compiler/rustc_ty_utils/src/lib.rs");
// 32: rustc_codegen_ssa (373 lines)
// include!("../rust/compiler/rustc_codegen_ssa/src/lib.rs");
// 33: rustc_monomorphize (55 lines)
// include!("../rust/compiler/rustc_monomorphize/src/lib.rs");
// 34: rustc_lint (684 lines)
// include!("../rust/compiler/rustc_lint/src/lib.rs");
// 35: rustc_feature (145 lines)
// include!("../rust/compiler/rustc_feature/src/lib.rs");
// 36: rustc_ast (50 lines)
// include!("../rust/compiler/rustc_ast/src/lib.rs");
// 37: rustc_incremental (29 lines)
// include!("../rust/compiler/rustc_incremental/src/lib.rs");
// 38: rustc_hashes (131 lines)
// include!("../rust/compiler/rustc_hashes/src/lib.rs");
// 39: rustc_mir_build (33 lines)
// include!("../rust/compiler/rustc_mir_build/src/lib.rs");
// 40: rustc_hir_id (196 lines)
// include!("../rust/compiler/rustc_hir_id/src/lib.rs");
// 41: rustc_data_structures (151 lines)
// include!("../rust/compiler/rustc_data_structures/src/lib.rs");
// 42: rustc_mir_transform (809 lines)
// include!("../rust/compiler/rustc_mir_transform/src/lib.rs");
// 43: std (771 lines)
// include!("../rust/library/std/src/lib.rs");
// 44: rustc_infer (30 lines)
// include!("../rust/compiler/rustc_infer/src/lib.rs");
// 45: test (795 lines)
// include!("../rust/library/test/src/lib.rs");
// 46: rustc_query_system (18 lines)
// include!("../rust/compiler/rustc_query_system/src/lib.rs");
// 47: rustc_log (244 lines)
// include!("../rust/compiler/rustc_log/src/lib.rs");
// 48: rustc_mir_dataflow (41 lines)
// include!("../rust/compiler/rustc_mir_dataflow/src/lib.rs");
// 49: rustc_attr_parsing (115 lines)
// include!("../rust/compiler/rustc_attr_parsing/src/lib.rs");
// 50: rustc_session (39 lines)
// include!("../rust/compiler/rustc_session/src/lib.rs");
// 51: rustc_macros (194 lines)
// include!("../rust/compiler/rustc_macros/src/lib.rs");
// 52: rustc_interface (25 lines)
// include!("../rust/compiler/rustc_interface/src/lib.rs");
// 53: rustc_ast_ir (313 lines)
// include!("../rust/compiler/rustc_ast_ir/src/lib.rs");
// 54: rustc_metadata (38 lines)
// include!("../rust/compiler/rustc_metadata/src/lib.rs");
// 55: rustc_codegen_gcc (481 lines)
// include!("../rust/compiler/rustc_codegen_gcc/src/lib.rs");
// 56: rustc_errors (2073 lines)
// include!("../rust/compiler/rustc_errors/src/lib.rs");
// 57: rustc_borrowck (2737 lines)
// include!("../rust/compiler/rustc_borrowck/src/lib.rs");
// 58: backtrace (267 lines)
// include!("../rust/library/backtrace/src/lib.rs");
// 59: rustc_builtin_macros (150 lines)
// include!("../rust/compiler/rustc_builtin_macros/src/lib.rs");
// 60: rustc_middle (98 lines)
// include!("../rust/compiler/rustc_middle/src/lib.rs");
// 61: rustc_hir_analysis (295 lines)
// include!("../rust/compiler/rustc_hir_analysis/src/lib.rs");
// 62: rustc_lint_defs (967 lines)
// include!("../rust/compiler/rustc_lint_defs/src/lib.rs");
// 63: rustc_type_ir (434 lines)
// include!("../rust/compiler/rustc_type_ir/src/lib.rs");

// Topological order complete!
// Total: 1 entry point + 63 compiler components
