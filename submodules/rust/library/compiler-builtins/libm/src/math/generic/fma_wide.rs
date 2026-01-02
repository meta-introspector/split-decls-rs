mkuse!{use crate :: support :: { CastFrom , CastInto , DFloat , Float , FpResult , HFloat , IntTy , MinInt , Round , Status , } ;}

macro_rules! fma_wide_round_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fma_wide_round in module {}", module_path!());
    };
}

mkfn!{
    fma_wide_round_introspect!();
    # [doc = " Fma implementation when a hardware-backed larger float type is available. For `f32` and `f64`,"] # [doc = " `f64` has enough precision to represent the `f32` in its entirety, except for double rounding."] # [inline] pub fn fma_wide_round < F , B > (x : F , y : F , z : F , round : Round) -> FpResult < F > where F : Float + HFloat < D = B > , B : Float + DFloat < H = F > , B :: Int : CastInto < i32 > , i32 : CastFrom < i32 > , { let one = IntTy :: < B > :: ONE ; let xy : B = x . widen () * y . widen () ; let mut result : B = xy + z . widen () ; let mut ui : B :: Int = result . to_bits () ; let re = result . ex () ; let zb : B = z . widen () ; let prec_diff = B :: SIG_BITS - F :: SIG_BITS ; let excess_prec = ui & ((one << prec_diff) - one) ; let halfway = one << (prec_diff - 1) ; if excess_prec != halfway || re == B :: EXP_SAT || (result - xy == zb && result - zb == xy) || round != Round :: Nearest { let min_inexact_exp = (B :: EXP_BIAS as i32 + F :: EXP_MIN_SUBNORM) as u32 ; let max_inexact_exp = (B :: EXP_BIAS as i32 + F :: EXP_MIN) as u32 ; let mut status = Status :: OK ; if (min_inexact_exp .. max_inexact_exp) . contains (& re) && status . inexact () { status . set_inexact (false) ; result = xy + z . widen () ; if status . inexact () { status . set_underflow (true) ; } else { status . set_inexact (true) ; } } return FpResult { val : result . narrow () , status , } ; } let neg = ui >> (B :: BITS - 1) != IntTy :: < B > :: ZERO ; let err = if neg == (zb > xy) { xy - result + zb } else { zb - result + xy } ; if neg == (err < B :: ZERO) { ui += one ; } else { ui -= one ; } FpResult :: ok (B :: from_bits (ui) . narrow ()) }
}