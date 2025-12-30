// Generated macro for impl_163 (impl)
macro_rules! Depcrate_run_cfgimpl_163 {
() => {
// Module: crate::run_cfg
// Provides: {"impl_163"}
// Dependencies: {}
impl TestEnv { fn from_env (ctx : & CheckCtx) -> Self { let id = ctx . fn_ident ; let op = id . math_op () ; let will_run_mp = cfg ! (feature = "build-mpfr") ; let large_float_ty = match op . float_ty { FloatTy :: F16 | FloatTy :: F32 => false , FloatTy :: F64 | FloatTy :: F128 => true , } ; let will_run_extensive = EXTENSIVE . contains (& id) ; let input_count = op . rust_sig . args . len () ; Self { slow_platform : slow_platform () , large_float_ty , should_run_extensive : will_run_extensive , mp_tests_enabled : will_run_mp , input_count , } } }
};
}
