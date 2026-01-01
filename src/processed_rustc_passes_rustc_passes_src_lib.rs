/* FP:lib.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_lib_USE_0001
/* FP:lib.rs-0002 */ # [allow (internal_features)] # [doc (html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/")] # [doc (rust_logo)] # [feature (if_let_guard)] # [feature (map_try_insert)] # [feature (rustdoc_internals)] use crate :: rustc_complete :: util :: Providers ;
/* FP:lib.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_lib_MOD_0002
/* FP:lib.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_lib_MOD_0003
/* FP:lib.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_lib_MOD_0004
/* FP:lib.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_lib_MOD_0005
/* FP:lib.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_lib_MOD_0006
/* FP:lib.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_lib_MOD_0007
/* FP:lib.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_lib_MOD_0008
/* FP:lib.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_lib_MOD_0009
/* FP:lib.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_lib_MOD_0010
/* FP:lib.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_lib_MOD_0011
/* FP:lib.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_lib_MOD_0012
/* FP:lib.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_lib_MOD_0013
/* FP:lib.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_lib_MOD_0014
/* FP:lib.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_lib_MOD_0015
/* FP:lib.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_lib_MOD_0016
/* FP:lib.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_lib_MOD_0017
/* FP:lib.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_lib_MOD_0018
/* FP:lib.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_lib_MOD_0019
/* FP:lib.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_lib_MACRO_0020
/* FP:lib.rs-0040 */ rustc_fluent_macro :: fluent_messages ! { "../messages.ftl" }
/* FP:lib.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_lib_FN_0021
/* FP:lib.rs-0042 */ pub fn provide (providers : & mut Providers) { check_attr :: provide (providers) ; dead :: provide (providers) ; debugger_visualizer :: provide (providers) ; diagnostic_items :: provide (providers) ; entry :: provide (providers) ; lang_items :: provide (providers) ; lib_features :: provide (providers) ; liveness :: provide (providers) ; reachable :: provide (providers) ; stability :: provide (providers) ; upvars :: provide (providers) ; check_export :: provide (providers) ; }