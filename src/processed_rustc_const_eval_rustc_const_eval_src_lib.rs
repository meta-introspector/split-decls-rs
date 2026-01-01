/* FP:lib.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_lib_MOD_0001
/* FP:lib.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_lib_MOD_0002
/* FP:lib.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_lib_MOD_0003
/* FP:lib.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_lib_MOD_0004
/* FP:lib.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_lib_MOD_0005
/* FP:lib.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_lib_USE_0006
/* FP:lib.rs-0012 */ use std :: sync :: atomic :: AtomicBool ;
/* FP:lib.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_lib_USE_0007
/* FP:lib.rs-0014 */ use crate :: rustc_complete :: ty ;
/* FP:lib.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_lib_USE_0008
/* FP:lib.rs-0016 */ use crate :: rustc_complete :: util :: Providers ;
/* FP:lib.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_lib_USE_0009
/* FP:lib.rs-0018 */ pub use self :: errors :: ReportErrorExt ;
/* FP:lib.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_lib_MACRO_0010
/* FP:lib.rs-0020 */ rustc_fluent_macro :: fluent_messages ! { "../messages.ftl" }
/* FP:lib.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_lib_FN_0011
/* FP:lib.rs-0022 */ pub fn provide (providers : & mut Providers) { const_eval :: provide (providers) ; providers . tag_for_variant = const_eval :: tag_for_variant_provider ; providers . eval_to_const_value_raw = const_eval :: eval_to_const_value_raw_provider ; providers . eval_to_allocation_raw = const_eval :: eval_to_allocation_raw_provider ; providers . eval_static_initializer = const_eval :: eval_static_initializer_provider ; providers . hooks . const_caller_location = util :: caller_location :: const_caller_location_provider ; providers . eval_to_valtree = | tcx , ty :: PseudoCanonicalInput { typing_env , value } | { const_eval :: eval_to_valtree (tcx , typing_env , value) } ; providers . hooks . try_destructure_mir_constant_for_user_output = const_eval :: try_destructure_mir_constant_for_user_output ; providers . valtree_to_const_val = | tcx , cv | const_eval :: valtree_to_const_value (tcx , ty :: TypingEnv :: fully_monomorphized () , cv) ; providers . check_validity_requirement = | tcx , (init_kind , param_env_and_ty) | { util :: check_validity_requirement (tcx , init_kind , param_env_and_ty) } ; providers . hooks . validate_scalar_in_layout = | tcx , scalar , layout | util :: validate_scalar_in_layout (tcx , scalar , layout) ; }
/* FP:lib.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_lib_STATIC_0012
/* FP:lib.rs-0024 */ # [doc = " `rustc_driver::main` installs a handler that will set this to `true` if"] # [doc = " the compiler has been sent a request to shut down, such as by a Ctrl-C."] # [doc = " This static lives here because it is only read by the interpreter."] pub static CTRL_C_RECEIVED : AtomicBool = AtomicBool :: new (false) ;