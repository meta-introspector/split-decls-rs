// Generated macro for unop_common (function)
macro_rules! Depcrate_precisionunop_common {
() => {
// Module: crate::precision
// Provides: {"unop_common"}
// Dependencies: {}
fn unop_common < F1 : Float , F2 : Float > (input : (F1 ,) , actual : F2 , expected : F2 , ctx : & CheckCtx ,) -> CheckAction { if ctx . base_name == BaseName :: Acosh && input . 0 < F1 :: NEG_ONE && ! (expected . is_nan () && actual . is_nan ()) { if ctx . basis == CheckBasis :: Musl { return XFAIL_NOCHECK ; } return XFAIL ("acoshf undefined") ; } if (ctx . base_name == BaseName :: Lgamma || ctx . base_name == BaseName :: LgammaR) && input . 0 < F1 :: ZERO && ! input . 0 . is_infinite () { return XFAIL_NOCHECK ; } if ctx . base_name == BaseName :: Fabs && input . 0 . is_nan () { if cfg ! (target_arch = "x86") && ctx . basis == CheckBasis :: Musl && actual . is_nan () { return XFAIL_NOCHECK ; } if ctx . basis == CheckBasis :: Mpfr { return DEFAULT ; } if actual . biteq (expected) { return CheckAction :: Custom (Ok (())) ; } else { return CheckAction :: Custom (Err (anyhow :: anyhow ! ("NaNs have different bitpatterns"))) ; } } DEFAULT }
};
}
