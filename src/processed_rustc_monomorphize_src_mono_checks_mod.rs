/* FP:mod.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_mono_checks_mod_USE_0001
/* FP:mod.rs-0002 */ use crate :: rustc_complete :: query :: Providers ;
/* FP:mod.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_mono_checks_mod_USE_0002
/* FP:mod.rs-0004 */ use crate :: rustc_complete :: ty :: { Instance , TyCtxt } ;
/* FP:mod.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_mono_checks_mod_MOD_0003
/* FP:mod.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_mono_checks_mod_MOD_0004
/* FP:mod.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_mono_checks_mod_FN_0005
/* FP:mod.rs-0010 */ fn check_mono_item < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx >) { let body = tcx . instance_mir (instance . def) ; abi_check :: check_feature_dependent_abi (tcx , instance , body) ; move_check :: check_moves (tcx , instance , body) ; }
/* FP:mod.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_monomorphize_src_mono_checks_mod_FN_0006
/* FP:mod.rs-0012 */ pub (super) fn provide (providers : & mut Providers) { * providers = Providers { check_mono_item , skip_move_check_fns : move_check :: skip_move_check_fns , .. * providers } }