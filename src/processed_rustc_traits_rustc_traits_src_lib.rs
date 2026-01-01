/* FP:lib.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_lib_MOD_0001
/* FP:lib.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_lib_MOD_0002
/* FP:lib.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_lib_MOD_0003
/* FP:lib.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_lib_MOD_0004
/* FP:lib.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_lib_MOD_0005
/* FP:lib.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_lib_MOD_0006
/* FP:lib.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_lib_MOD_0007
/* FP:lib.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_lib_MOD_0008
/* FP:lib.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_lib_USE_0009
/* FP:lib.rs-0018 */ use crate :: rustc_complete :: query :: Providers ;
/* FP:lib.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_lib_USE_0010
/* FP:lib.rs-0020 */ pub use crate :: rustc_trait_selection :: traits :: query :: type_op :: ascribe_user_type :: type_op_ascribe_user_type_with_span ;
/* FP:lib.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_lib_USE_0011
/* FP:lib.rs-0022 */ pub use type_op :: type_op_prove_predicate_with_cause ;
/* FP:lib.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_lib_FN_0012
/* FP:lib.rs-0024 */ pub fn provide (p : & mut Providers) { dropck_outlives :: provide (p) ; evaluate_obligation :: provide (p) ; implied_outlives_bounds :: provide (p) ; normalize_projection_ty :: provide (p) ; normalize_erasing_regions :: provide (p) ; type_op :: provide (p) ; p . codegen_select_candidate = codegen :: codegen_select_candidate ; p . coroutine_hidden_types = coroutine_witnesses :: coroutine_hidden_types ; }