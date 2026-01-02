mkuse!{use super :: { exp , expm1 , k_expo2 } ;}

macro_rules! cosh_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cosh in module {}", module_path!());
    };
}

mkfn!{
    cosh_introspect!();
    # [doc = " Hyperbolic cosine (f64)"] # [doc = ""] # [doc = " Computes the hyperbolic cosine of the argument x."] # [doc = " Is defined as `(exp(x) + exp(-x))/2`"] # [doc = " Angles are specified in radians."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn cosh (mut x : f64) -> f64 { let mut ix = x . to_bits () ; ix &= 0x7fffffffffffffff ; x = f64 :: from_bits (ix) ; let w = ix >> 32 ; if w < 0x3fe62e42 { if w < 0x3ff00000 - (26 << 20) { let x1p120 = f64 :: from_bits (0x4770000000000000) ; force_eval ! (x + x1p120) ; return 1. ; } let t = expm1 (x) ; return 1. + t * t / (2. * (1. + t)) ; } if w < 0x40862e42 { let t = exp (x) ; return 0.5 * (t + 1. / t) ; } k_expo2 (x) }
}