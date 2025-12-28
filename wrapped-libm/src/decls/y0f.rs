macro_rules! y0f {
    () => {
        # [doc = " Zeroth order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the second kind (f32)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn y0f (x : f32) -> f32 { let z : f32 ; let u : f32 ; let v : f32 ; let ix : u32 ; ix = x . to_bits () ; if (ix & 0x7fffffff) == 0 { return - 1.0 / 0.0 ; } if (ix >> 31) != 0 { return 0.0 / 0.0 ; } if ix >= 0x7f800000 { return 1.0 / x ; } if ix >= 0x40000000 { return common (ix , x , true) ; } if ix >= 0x39000000 { z = x * x ; u = U00 + z * (U01 + z * (U02 + z * (U03 + z * (U04 + z * (U05 + z * U06))))) ; v = 1.0 + z * (V01 + z * (V02 + z * (V03 + z * V04))) ; return u / v + TPI * (j0f (x) * logf (x)) ; } return U00 + TPI * logf (x) ; }
    };
}

y0f!();