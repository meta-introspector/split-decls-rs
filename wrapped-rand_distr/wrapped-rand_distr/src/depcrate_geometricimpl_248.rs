// Generated macro for impl_248 (impl)
macro_rules! Depcrate_geometricimpl_248 {
() => {
// Module: crate::geometric
// Provides: {"impl_248"}
// Dependencies: {}
impl Distribution < u64 > for Geometric { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> u64 { if self . p >= 2.0 / 3.0 { let mut failures = 0 ; loop { let u = rng . random :: < f64 > () ; if u <= self . p { break ; } failures += 1 ; } return failures ; } if self . p == 0.0 { return u64 :: MAX ; } let Geometric { p , pi , k } = * self ; let d = { let mut failures = 0 ; while rng . random :: < f64 > () < pi { failures += 1 ; } failures } ; let m = loop { let m = rng . random :: < u64 > () & ((1 << k) - 1) ; let p_reject = if m <= i32 :: MAX as u64 { (1.0 - p) . powi (m as i32) } else { (1.0 - p) . powf (m as f64) } ; let u = rng . random :: < f64 > () ; if u < p_reject { break m ; } } ; (d << k) + m } }
};
}
