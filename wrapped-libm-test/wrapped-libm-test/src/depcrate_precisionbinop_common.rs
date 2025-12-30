// Generated macro for binop_common (function)
macro_rules! Depcrate_precisionbinop_common {
() => {
// Module: crate::precision
// Provides: {"binop_common"}
// Dependencies: {}
fn binop_common < F1 : Float , F2 : Float > (input : (F1 , F1) , actual : F2 , expected : F2 , ctx : & CheckCtx ,) -> CheckAction { if ctx . base_name == BaseName :: Copysign && ctx . basis == CheckBasis :: Mpfr && input . 1 . is_nan () { return SKIP ; } if ctx . base_name == BaseName :: FmaximumNum && ctx . basis == CheckBasis :: Mpfr && ((input . 0 . is_nan () && actual . is_nan () && expected . is_nan ()) || input . 1 . is_nan ()) { return XFAIL_NOCHECK ; } if ctx . base_name == BaseName :: Fmin && input . 0 . biteq (F1 :: NEG_ZERO) && input . 1 . biteq (F1 :: ZERO) && expected . biteq (F2 :: NEG_ZERO) && actual . biteq (F2 :: ZERO) { return XFAIL ("fmin signed zeroes") ; } if ctx . base_name == BaseName :: Fmax && input . 0 . biteq (F1 :: NEG_ZERO) && input . 1 . biteq (F1 :: ZERO) && expected . biteq (F2 :: ZERO) && actual . biteq (F2 :: NEG_ZERO) { return XFAIL ("fmax signed zeroes") ; } if (ctx . base_name == BaseName :: Fmax || ctx . base_name == BaseName :: Fmin) && ctx . basis == Musl && (input . 0 . is_nan () ^ input . 1 . is_nan ()) && expected . is_nan () { return XFAIL ("fmax/fmin musl NaN") ; } DEFAULT }
};
}
