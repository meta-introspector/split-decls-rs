// rustc_complete.rs - No stubs, only real types from macro system

// Include wrap_types only if it contains non-conflicting definitions
// include!("wrap_types.rs");

// Add PResult type alias
pub type PResult<T> = Result<T, ()>;

// Define all the use_crate_rustc_*!() macros that unified_rustc_wrapped.rs expects
macro_rules! use_crate_rustc_mir_dataflow { () => { pub use rustc_mir_dataflow::*; }; }
macro_rules! use_crate_rustc_infer { () => { pub use rustc_infer::*; }; }
macro_rules! use_crate_rustc_parse_format { () => { pub use rustc_parse_format::*; }; }
macro_rules! use_crate_rustc_hir_analysis { () => { pub use rustc_hir_analysis::*; }; }
macro_rules! use_crate_rustc_ty_utils { () => { pub use rustc_ty_utils::*; }; }
macro_rules! use_crate_rustc_middle { () => { pub use rustc_middle::*; }; }
macro_rules! use_crate_rustc_query_system { () => { pub use rustc_query_system::*; }; }
macro_rules! use_crate_rustc_hir_typeck { () => { pub use rustc_hir_typeck::*; }; }
macro_rules! use_crate_rustc_hir { () => { pub use rustc_hir::*; }; }
macro_rules! use_crate_rustc_attr_parsing { () => { pub use rustc_attr_parsing::*; }; }
macro_rules! use_crate_rustc_builtin_macros { () => { pub use rustc_builtin_macros::*; }; }
macro_rules! use_crate_rustc_interface { () => { pub use rustc_interface::*; }; }
macro_rules! use_crate_rustc_mir_build { () => { pub use rustc_mir_build::*; }; }
macro_rules! use_crate_rustc_symbol_mangling { () => { pub use rustc_symbol_mangling::*; }; }
macro_rules! use_crate_rustc_codegen_gcc { () => { /* Optional backend - stub */ }; }
macro_rules! use_crate_rustc_expand { () => { pub use rustc_expand::*; }; }
macro_rules! use_crate_rustc_session { () => { pub use rustc_session::*; }; }
macro_rules! use_crate_rustc_mir_transform { () => { pub use rustc_mir_transform::*; }; }
macro_rules! use_crate_rustc_index { () => { pub use rustc_index::*; }; }
macro_rules! use_crate_rustc_next_trait_solver { () => { pub use rustc_next_trait_solver::*; }; }
macro_rules! use_crate_rustc_parse { () => { pub use rustc_parse::*; }; }
macro_rules! use_crate_rustc_codegen_llvm { () => { pub use rustc_codegen_llvm::*; }; }
macro_rules! use_crate_rustc_lint { () => { pub use rustc_lint::*; }; }
macro_rules! use_crate_rustc_public { () => { pub use rustc_public::*; }; }
macro_rules! use_crate_rustc_passes { () => { pub use rustc_passes::*; }; }
macro_rules! use_crate_rustc_codegen_ssa { () => { pub use rustc_codegen_ssa::*; }; }
macro_rules! use_crate_rustc_trait_selection { () => { pub use rustc_trait_selection::*; }; }
macro_rules! use_crate_rustc_codegen_cranelift { () => { /* Optional backend - stub */ }; }
macro_rules! use_crate_rustc_const_eval { () => { pub use rustc_const_eval::*; }; }
macro_rules! use_crate_rustc_pattern_analysis { () => { pub use rustc_pattern_analysis::*; }; }
macro_rules! use_crate_rustc_span { () => { pub use rustc_span::*; }; }
macro_rules! use_crate_rustc_ast { () => { pub use rustc_ast::*; }; }
macro_rules! use_crate_rustc_type_ir { () => { pub use rustc_type_ir::*; }; }
macro_rules! use_crate_rustc_resolve { () => { pub use rustc_resolve::*; }; }
macro_rules! use_crate_rustc_borrowck { () => { pub use rustc_borrowck::*; }; }
macro_rules! use_crate_rustc_data_structures { () => { pub use rustc_data_structures::*; }; }
macro_rules! use_crate_rustc_serialize { () => { pub use rustc_serialize::*; }; }
macro_rules! use_crate_rustc_errors { () => { pub use rustc_errors::*; }; }
macro_rules! use_crate_rustc_metadata { () => { pub use rustc_metadata::*; }; }
macro_rules! use_crate_rustc_transmute { () => { pub use rustc_transmute::*; }; }
macro_rules! use_crate_rustc_thread_pool { () => { pub use rustc_thread_pool::*; }; }
macro_rules! use_crate_rustc_incremental { () => { pub use rustc_incremental::*; }; }
macro_rules! use_crate_rustc_ast_ir { () => { pub use rustc_ast_ir::*; }; }
macro_rules! use_crate_rustc_traits { () => { pub use rustc_traits::*; }; }
macro_rules! use_crate_rustc_macros { () => { pub use rustc_macros::*; }; }
macro_rules! use_crate_rustc_ast_lowering { () => { pub use rustc_ast_lowering::*; }; }
macro_rules! use_crate_rustc_driver_impl { () => { pub use rustc_driver_impl::*; }; }
macro_rules! use_crate_rustc_query_impl { () => { pub use rustc_query_impl::*; }; }
macro_rules! use_crate_rustc_monomorphize { () => { pub use rustc_monomorphize::*; }; }
macro_rules! use_crate_rustc_ast_passes { () => { pub use rustc_ast_passes::*; }; }
macro_rules! use_crate_rustc_lint_defs { () => { pub use rustc_lint_defs::*; }; }
macro_rules! use_crate_rustc_arena { () => { pub use rustc_arena::*; }; }
macro_rules! use_crate_rustc_abi { () => { pub use rustc_abi::*; }; }
macro_rules! use_crate_rustc_sanitizers { () => { pub use rustc_sanitizers::*; }; }
macro_rules! use_crate_rustc_privacy { () => { pub use rustc_privacy::*; }; }
macro_rules! use_crate_rustc_hir_id { () => { pub use rustc_hir_id::*; }; }
macro_rules! use_crate_rustc_baked_icu_data { () => { pub use rustc_baked_icu_data::*; }; }
macro_rules! use_crate_rustc_error_messages { () => { pub use rustc_error_messages::*; }; }
macro_rules! use_crate_rustc_feature { () => { pub use rustc_feature::*; }; }
macro_rules! use_crate_rustc_graphviz { () => { pub use rustc_graphviz::*; }; }
macro_rules! use_crate_rustc_llvm { () => { pub use rustc_llvm::*; }; }
macro_rules! use_crate_rustc_public_bridge { () => { pub use rustc_public_bridge::*; }; }
macro_rules! use_crate_rustc_ast_pretty { () => { pub use rustc_ast_pretty::*; }; }
macro_rules! use_crate_rustc_hir_pretty { () => { pub use rustc_hir_pretty::*; }; }
macro_rules! use_crate_rustc_lexer { () => { pub use rustc_lexer::*; }; }
macro_rules! use_crate_rustc_index_macros { () => { pub use rustc_index_macros::*; }; }
macro_rules! use_crate_rustc_log { () => { pub use rustc_log::*; }; }
macro_rules! use_crate_rustc_hashes { () => { pub use rustc_hashes::*; }; }
macro_rules! use_crate_rustc_fs_util { () => { pub use rustc_fs_util::*; }; }
macro_rules! use_crate_rustc_fluent_macro { () => { pub use rustc_fluent_macro::*; }; }
macro_rules! use_crate_rustc_type_ir_macros { () => { pub use rustc_type_ir_macros::*; }; }
macro_rules! use_crate_rustc_error_codes { () => { pub use rustc_error_codes::*; }; }
macro_rules! use_crate_rustc_driver { () => { pub use rustc_driver::*; }; }
