macro_rules! y0 {
    () => {
        # [doc = " Zeroth order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the second kind (f64)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn y0 (x : f64) -> f64 { let z : f64 ; let u : f64 ; let v : f64 ; let ix : u32 ; let lx : u32 ; ix = get_high_word (x) ; lx = get_low_word (x) ; if ((ix << 1) | lx) == 0 { return - 1.0 / 0.0 ; } if (ix >> 31) != 0 { return 0.0 / 0.0 ; } if ix >= 0x7ff00000 { return 1.0 / x ; } if ix >= 0x40000000 { return common (ix , x , true) ; } if ix >= 0x3e400000 { z = x * x ; u = U00 + z * (U01 + z * (U02 + z * (U03 + z * (U04 + z * (U05 + z * U06))))) ; v = 1.0 + z * (V01 + z * (V02 + z * (V03 + z * V04))) ; return u / v + TPI * (j0 (x) * log (x)) ; } return U00 + TPI * log (x) ; }
    };
}

y0!();