mkuse!{use crate :: support :: { CastFrom , Float , Int , MinInt } ;}

macro_rules! fmod_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmod in module {}", module_path!());
    };
}

mkfn!{
    fmod_introspect!();
    # [inline] pub fn fmod < F : Float > (x : F , y : F) -> F { let _1 = F :: Int :: ONE ; let sx = x . to_bits () & F :: SIGN_MASK ; let ux = x . to_bits () & ! F :: SIGN_MASK ; let uy = y . to_bits () & ! F :: SIGN_MASK ; let x_nan_or_inf = ux & F :: EXP_MASK == F :: EXP_MASK ; let y_nan_or_zero = uy . wrapping_sub (_1) & F :: EXP_MASK == F :: EXP_MASK ; if x_nan_or_inf | y_nan_or_zero { return (x * y) / (x * y) ; } if ux < uy { return x ; } let (num , ex) = into_sig_exp :: < F > (ux) ; let (div , ey) = into_sig_exp :: < F > (uy) ; let rem = reduction (num , ex - ey , div) ; if rem . is_zero () { return F :: from_bits (sx) ; } ; let shift = ey . min (F :: SIG_BITS - rem . ilog2 ()) ; let bits = (rem << shift) + (F :: Int :: cast_from (ey - shift) << F :: SIG_BITS) ; F :: from_bits (sx + bits) }
}

macro_rules! into_sig_exp_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function into_sig_exp in module {}", module_path!());
    };
}

mkfn!{
    into_sig_exp_introspect!();
    # [doc = " Given the bits of a finite float, return a tuple of"] # [doc = "  - the mantissa with the implicit bit (0 if subnormal, 1 otherwise)"] # [doc = "  - the additional exponent past 1, (0 for subnormal, 0 or more otherwise)"] fn into_sig_exp < F : Float > (mut bits : F :: Int) -> (F :: Int , u32) { bits &= ! F :: SIGN_MASK ; let sat = bits . checked_sub (F :: IMPLICIT_BIT) . unwrap_or (F :: Int :: ZERO) ; (bits - (sat & F :: EXP_MASK) , u32 :: cast_from (sat >> F :: SIG_BITS) ,) }
}

macro_rules! reduction_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function reduction in module {}", module_path!());
    };
}

mkfn!{
    reduction_introspect!();
    # [doc = " Compute the remainder `(x * 2.pow(e)) % y` without overflow."] fn reduction < I : Int > (mut x : I , e : u32 , y : I) -> I { x %= y ; for _ in 0 .. e { x <<= 1 ; x = x . checked_sub (y) . unwrap_or (x) ; } x }
}