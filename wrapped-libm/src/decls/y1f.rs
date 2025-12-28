macro_rules! y1f {
    () => {
        # [doc = " First order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the second kind (f32)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn y1f (x : f32) -> f32 { let z : f32 ; let u : f32 ; let v : f32 ; let ix : u32 ; ix = x . to_bits () ; if (ix & 0x7fffffff) == 0 { return - 1.0 / 0.0 ; } if (ix >> 31) != 0 { return 0.0 / 0.0 ; } if ix >= 0x7f800000 { return 1.0 / x ; } if ix >= 0x40000000 { return common (ix , x , true , false) ; } if ix < 0x33000000 { return - TPI / x ; } z = x * x ; u = U0 [0] + z * (U0 [1] + z * (U0 [2] + z * (U0 [3] + z * U0 [4]))) ; v = 1.0 + z * (V0 [0] + z * (V0 [1] + z * (V0 [2] + z * (V0 [3] + z * V0 [4])))) ; return x * (u / v) + TPI * (j1f (x) * logf (x) - 1.0 / x) ; }
    };
}

y1f!();