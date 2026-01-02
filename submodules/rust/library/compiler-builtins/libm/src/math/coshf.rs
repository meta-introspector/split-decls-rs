mkuse!{use super :: { expf , expm1f , k_expo2f } ;}

macro_rules! coshf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function coshf in module {}", module_path!());
    };
}

mkfn!{
    coshf_introspect!();
    # [doc = " Hyperbolic cosine (f64)"] # [doc = ""] # [doc = " Computes the hyperbolic cosine of the argument x."] # [doc = " Is defined as `(exp(x) + exp(-x))/2`"] # [doc = " Angles are specified in radians."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn coshf (mut x : f32) -> f32 { let x1p120 = f32 :: from_bits (0x7b800000) ; let mut ix = x . to_bits () ; ix &= 0x7fffffff ; x = f32 :: from_bits (ix) ; let w = ix ; if w < 0x3f317217 { if w < (0x3f800000 - (12 << 23)) { force_eval ! (x + x1p120) ; return 1. ; } let t = expm1f (x) ; return 1. + t * t / (2. * (1. + t)) ; } if w < 0x42b17217 { let t = expf (x) ; return 0.5 * (t + 1. / t) ; } k_expo2f (x) }
}