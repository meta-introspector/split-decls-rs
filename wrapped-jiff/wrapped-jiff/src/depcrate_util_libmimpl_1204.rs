// Generated macro for impl_1204 (impl)
macro_rules! Depcrate_util_libmimpl_1204 {
() => {
// Module: crate::util::libm
// Provides: {"impl_1204"}
// Dependencies: {}
impl Float for f64 { fn abs (self) -> f64 { if self . is_sign_negative () { - self } else { self } } fn ceil (self) -> f64 { let x = self ; let u : u64 = x . to_bits () ; let e : i64 = (u >> 52 & 0x7ff) as i64 ; let y : f64 ; if e >= 0x3ff + 52 || x == 0. { return x ; } y = if (u >> 63) != 0 { x - TOINT64 + TOINT64 - x } else { x + TOINT64 - TOINT64 - x } ; if e < 0x3ff { core :: hint :: black_box (y) ; return if (u >> 63) != 0 { - 0. } else { 1. } ; } if y < 0. { x + y + 1. } else { x + y } } fn floor (self) -> f64 { let x = self ; let ui = x . to_bits () ; let e = ((ui >> 52) & 0x7ff) as i32 ; if (e >= 0x3ff + 52) || (x == 0.) { return x ; } let y = if (ui >> 63) != 0 { x - TOINT64 + TOINT64 - x } else { x + TOINT64 - TOINT64 - x } ; if e < 0x3ff { core :: hint :: black_box (y) ; return if (ui >> 63) != 0 { - 1. } else { 0. } ; } if y > 0. { x + y - 1. } else { x + y } } fn round (self) -> f64 { (self + copysign64 (0.5 - 0.25 * f64 :: EPSILON , self)) . trunc () } fn signum (self) -> f64 { if self . is_nan () { Self :: NAN } else { copysign64 (1.0 , self) } } fn trunc (self) -> f64 { let x = self ; let x1p120 = f64 :: from_bits (0x4770000000000000) ; let mut i : u64 = x . to_bits () ; let mut e : i64 = (i >> 52 & 0x7ff) as i64 - 0x3ff + 12 ; let m : u64 ; if e >= 52 + 12 { return x ; } if e < 12 { e = 1 ; } m = - 1i64 as u64 >> e ; if (i & m) == 0 { return x ; } core :: hint :: black_box (x + x1p120) ; i &= ! m ; f64 :: from_bits (i) } fn fract (self) -> f64 { self - self . trunc () } }
};
}
