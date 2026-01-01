/* FP:lib.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_lib_USE_0001
/* FP:lib.rs-0002 */ # [allow (internal_features)] # [doc (html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/")] # [doc (rust_logo)] # [feature (assert_matches)] # [feature (associated_type_defaults)] # [feature (box_patterns)] # [feature (if_let_guard)] # [feature (iterator_try_collect)] # [feature (never_type)] # [feature (rustdoc_internals)] use crate :: rustc_complete :: query :: Providers ;
/* FP:lib.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_lib_MOD_0002
/* FP:lib.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_lib_MOD_0003
/* FP:lib.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_lib_MOD_0004
/* FP:lib.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_lib_MOD_0005
/* FP:lib.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_lib_MOD_0006
/* FP:lib.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_lib_MOD_0007
/* FP:lib.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_lib_MOD_0008
/* FP:lib.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_lib_MOD_0009
/* FP:lib.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_lib_MOD_0010
/* FP:lib.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_lib_MOD_0011
/* FP:lib.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_lib_MOD_0012
/* FP:lib.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_lib_MOD_0013
/* FP:lib.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_lib_MOD_0014
/* FP:lib.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_lib_MOD_0015
/* FP:lib.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_lib_MOD_0016
/* FP:lib.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_lib_MACRO_0017
/* FP:lib.rs-0034 */ rustc_fluent_macro :: fluent_messages ! { "../messages.ftl" }
/* FP:lib.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ty_utils_src_lib_FN_0018
/* FP:lib.rs-0036 */ pub fn provide (providers : & mut Providers) { abi :: provide (providers) ; assoc :: provide (providers) ; common_traits :: provide (providers) ; consts :: provide (providers) ; implied_bounds :: provide (providers) ; layout :: provide (providers) ; needs_drop :: provide (providers) ; opaque_types :: provide (providers) ; representability :: provide (providers) ; ty :: provide (providers) ; instance :: provide (providers) ; structural_match :: provide (providers) ; nested_bodies :: provide (providers) ; }