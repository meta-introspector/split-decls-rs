// Generated macro for impl_124 (impl)
macro_rules! Depcrate_precisionimpl_124 {
() => {
// Module: crate::precision
// Provides: {"impl_124"}
// Dependencies: {}
impl MaybeOverride < (f32 ,) > for SpecialCase { fn check_float < F : Float > (input : (f32 ,) , actual : F , expected : F , ctx : & CheckCtx) -> CheckAction { if ctx . base_name == BaseName :: Expm1 && ! input . 0 . is_infinite () && input . 0 > 80.0 && actual . is_infinite () && ! expected . is_infinite () { if ctx . basis == CheckBasis :: Musl { return XFAIL_NOCHECK ; } return XFAIL ("expm1 representable numbers") ; } if cfg ! (x86_no_sse) && ctx . base_name == BaseName :: Exp2 && ! expected . is_infinite () && actual . is_infinite () { return XFAIL ("586 exp2 representable numbers") ; } if ctx . base_name == BaseName :: Sinh && input . 0 . abs () > 80.0 && actual . is_nan () { if ctx . basis == CheckBasis :: Musl { return XFAIL_NOCHECK ; } return XFAIL ("sinh unexpected NaN") ; } if (ctx . base_name == BaseName :: Lgamma || ctx . base_name == BaseName :: LgammaR) && input . 0 > 4e36 && expected . is_infinite () && ! actual . is_infinite () { return XFAIL_NOCHECK ; } if ctx . base_name == BaseName :: J0 && input . 0 < - 1e34 { return XFAIL_NOCHECK ; } unop_common (input , actual , expected , ctx) } fn check_int < I : Int > (input : (f32 ,) , actual : I , expected : I , ctx : & CheckCtx) -> CheckAction { if ctx . basis == CheckBasis :: Mpfr && ctx . base_name == BaseName :: LgammaR && input . 0 == f32 :: NEG_INFINITY && actual . abs () == expected . abs () { return XFAIL ("lgammar integer result") ; } DEFAULT } }
};
}
