/* FP:lib.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_lib_MOD_0001
/* FP:lib.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_lib_MOD_0002
/* FP:lib.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_lib_MOD_0003
/* FP:lib.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_lib_USE_0004
/* FP:lib.rs-0008 */ pub use persist :: { LoadResult , copy_cgu_workproduct_to_incr_comp_cache_dir , finalize_session_directory , in_incr_comp_dir , in_incr_comp_dir_sess , load_query_result_cache , save_work_product_index , setup_dep_graph , } ;
/* FP:lib.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_lib_USE_0005
/* FP:lib.rs-0010 */ use crate :: rustc_complete :: util :: Providers ;
/* FP:lib.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_lib_FN_0006
/* FP:lib.rs-0012 */ # [allow (missing_docs)] pub fn provide (providers : & mut Providers) { providers . hooks . save_dep_graph = | tcx | tcx . sess . time ("serialize_dep_graph" , | | persist :: save_dep_graph (tcx)) ; }
/* FP:lib.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_lib_MACRO_0007
/* FP:lib.rs-0014 */ rustc_fluent_macro :: fluent_messages ! { "../messages.ftl" }