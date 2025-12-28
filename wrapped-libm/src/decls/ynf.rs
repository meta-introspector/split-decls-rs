macro_rules! ynf {
    () => {
        # [doc = " Integer order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the second kind (f32)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn ynf (n : i32 , x : f32) -> f32 { let mut ix : u32 ; let mut ib : u32 ; let nm1 : i32 ; let mut sign : bool ; let mut i : i32 ; let mut a : f32 ; let mut b : f32 ; let mut temp : f32 ; ix = x . to_bits () ; sign = (ix >> 31) != 0 ; ix &= 0x7fffffff ; if ix > 0x7f800000 { return x ; } if sign && ix != 0 { return 0.0 / 0.0 ; } if ix == 0x7f800000 { return 0.0 ; } if n == 0 { return y0f (x) ; } if n < 0 { nm1 = - (n + 1) ; sign = (n & 1) != 0 ; } else { nm1 = n - 1 ; sign = false ; } if nm1 == 0 { if sign { return - y1f (x) ; } else { return y1f (x) ; } } a = y0f (x) ; b = y1f (x) ; ib = b . to_bits () ; i = 0 ; while i < nm1 && ib != 0xff800000 { i += 1 ; temp = b ; b = (2.0 * (i as f32) / x) * b - a ; ib = b . to_bits () ; a = temp ; } if sign { - b } else { b } }
    };
}

ynf!();