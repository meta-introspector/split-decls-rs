// Generated macro for impl_133 (impl)
macro_rules! Depcrate_precisionimpl_133 {
() => {
// Module: crate::precision
// Provides: {"impl_133"}
// Dependencies: {}
impl MaybeOverride < (i32 , f32) > for SpecialCase { fn check_float < F : Float > (input : (i32 , f32) , actual : F , expected : F , ctx : & CheckCtx ,) -> CheckAction { if ctx . basis == Mpfr && ctx . base_name == BaseName :: Yn && input . 0 > 200 && ! expected . is_infinite () && actual . is_infinite () { return XFAIL ("ynf infinity mismatch") ; } int_float_common (input , actual , expected , ctx) } }
};
}
