// Generated macro for int_float_common (function)
macro_rules! Depcrate_precisionint_float_common {
() => {
// Module: crate::precision
// Provides: {"int_float_common"}
// Dependencies: {}
fn int_float_common < F1 : Float , F2 : Float > (input : (i32 , F1) , actual : F2 , expected : F2 , ctx : & CheckCtx ,) -> CheckAction { if ctx . basis == Mpfr && (ctx . base_name == BaseName :: Jn || ctx . base_name == BaseName :: Yn) && input . 1 == F1 :: NEG_INFINITY && actual == F2 :: ZERO && expected == F2 :: ZERO { return XFAIL ("we disagree with MPFR on the sign of zero") ; } if ctx . basis == Musl && ctx . fn_ident == Identifier :: Ynf && ! expected . is_infinite () && actual . is_infinite () && (expected . abs () . to_bits () . abs_diff (actual . abs () . to_bits ()) < F2 :: Int :: cast_from (10_000_000u32)) { return XFAIL_NOCHECK ; } if ctx . basis == Musl && (ctx . base_name == BaseName :: Jn || ctx . base_name == BaseName :: Yn) { if cfg ! (x86_no_sse) { return XFAIL_NOCHECK ; } if input . 0 > 4000 { return XFAIL_NOCHECK ; } else if input . 0 > 100 { return CheckAction :: AssertWithUlp (1_000_000) ; } } DEFAULT }
};
}
