// Generated macro for impl_125 (impl)
macro_rules! Depcrate_precisionimpl_125 {
() => {
// Module: crate::precision
// Provides: {"impl_125"}
// Dependencies: {}
impl MaybeOverride < (f64 ,) > for SpecialCase { fn check_float < F : Float > (input : (f64 ,) , actual : F , expected : F , ctx : & CheckCtx) -> CheckAction { if cfg ! (x86_no_sse) && (ctx . base_name == BaseName :: Rint || ctx . base_name == BaseName :: Roundeven) && (expected - actual) . abs () <= F :: ONE && (expected - actual) . abs () > F :: ZERO { return XFAIL ("i586 rint rounding mode") ; } if cfg ! (x86_no_sse) && (ctx . fn_ident == Identifier :: Exp10 || ctx . fn_ident == Identifier :: Exp2) { return XFAIL_NOCHECK ; } if ctx . base_name == BaseName :: J0 && input . 0 < - 1e300 { return XFAIL_NOCHECK ; } unop_common (input , actual , expected , ctx) } fn check_int < I : Int > (input : (f64 ,) , actual : I , expected : I , ctx : & CheckCtx) -> CheckAction { if ctx . basis == CheckBasis :: Mpfr && ctx . base_name == BaseName :: LgammaR && input . 0 == f64 :: NEG_INFINITY && actual . abs () == expected . abs () { return XFAIL ("lgammar integer result") ; } DEFAULT } }
};
}
