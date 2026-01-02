mkuse!{use super :: sqrt :: sqrt ;}
mkuse!{use super :: support :: Float ;}
mkitem!{const PIO2 : f64 = 1.570796326794896558e+00 ;}
mkitem!{const P_S0 : f32 = 1.6666586697e-01 ;}
mkitem!{const P_S1 : f32 = - 4.2743422091e-02 ;}
mkitem!{const P_S2 : f32 = - 8.6563630030e-03 ;}
mkitem!{const Q_S1 : f32 = - 7.0662963390e-01 ;}

macro_rules! r_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function r in module {}", module_path!());
    };
}

mkfn!{
    r_introspect!();
    fn r (z : f32) -> f32 { let p = z * (P_S0 + z * (P_S1 + z * P_S2)) ; let q = 1. + z * Q_S1 ; p / q }
}

macro_rules! asinf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function asinf in module {}", module_path!());
    };
}

mkfn!{
    asinf_introspect!();
    # [doc = " Arcsine (f32)"] # [doc = ""] # [doc = " Computes the inverse sine (arc sine) of the argument `x`."] # [doc = " Arguments to asin must be in the range -1 to 1."] # [doc = " Returns values in radians, in the range of -pi/2 to pi/2."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn asinf (mut x : f32) -> f32 { let x1p_120 = f64 :: from_bits (0x3870000000000000) ; let hx = x . to_bits () ; let ix = hx & 0x7fffffff ; if ix >= 0x3f800000 { if ix == 0x3f800000 { return ((x as f64) * PIO2 + x1p_120) as f32 ; } return 0. / (x - x) ; } if ix < 0x3f000000 { if (0x00800000 .. 0x39800000) . contains (& ix) { return x ; } return x + x * r (x * x) ; } let z = (1. - Float :: abs (x)) * 0.5 ; let s = sqrt (z as f64) ; x = (PIO2 - 2. * (s + s * (r (z) as f64))) as f32 ; if (hx >> 31) != 0 { - x } else { x } }
}