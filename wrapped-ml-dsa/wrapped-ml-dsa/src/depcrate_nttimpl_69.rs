// Generated macro for impl_69 (impl)
macro_rules! Depcrate_nttimpl_69 {
() => {
// Module: crate::ntt
// Provides: {"impl_69"}
// Dependencies: {}
impl Ntt for Polynomial { type Output = NttPolynomial ; fn ntt (& self) -> Self :: Output { let mut w = self . 0 . clone () ; let mut m = 0 ; for len in [128 , 64 , 32 , 16 , 8 , 4 , 2 , 1] { for start in (0 .. 256) . step_by (2 * len) { m += 1 ; let z = ZETA_POW_BITREV [m] ; for j in start .. (start + len) { let t = z * w [j + len] ; w [j + len] = w [j] - t ; w [j] = w [j] + t ; } } } NttPolynomial :: new (w) } }
};
}
