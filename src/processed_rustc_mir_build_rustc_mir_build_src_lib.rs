/* FP:lib.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_build_src_lib_MOD_0001
/* FP:lib.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_build_src_lib_MOD_0002
/* FP:lib.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_build_src_lib_MOD_0003
/* FP:lib.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_build_src_lib_MOD_0004
/* FP:lib.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_build_src_lib_MOD_0005
/* FP:lib.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_build_src_lib_USE_0006
/* FP:lib.rs-0012 */ use crate :: rustc_complete :: util :: Providers ;
/* FP:lib.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_build_src_lib_MACRO_0007
/* FP:lib.rs-0014 */ rustc_fluent_macro :: fluent_messages ! { "../messages.ftl" }
/* FP:lib.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_build_src_lib_FN_0008
/* FP:lib.rs-0016 */ pub fn provide (providers : & mut Providers) { providers . check_match = thir :: pattern :: check_match ; providers . lit_to_const = thir :: constant :: lit_to_const ; providers . closure_saved_names_of_captured_variables = builder :: closure_saved_names_of_captured_variables ; providers . check_unsafety = check_unsafety :: check_unsafety ; providers . check_tail_calls = check_tail_calls :: check_tail_calls ; providers . thir_body = thir :: cx :: thir_body ; }