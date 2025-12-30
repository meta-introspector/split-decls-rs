// Generated macro for impl_72 (impl)
macro_rules! Depcrate_nttimpl_72 {
() => {
// Module: crate::ntt
// Provides: {"impl_72"}
// Dependencies: {}
impl NttInverse for NttPolynomial { type Output = Polynomial ; fn ntt_inverse (& self) -> Self :: Output { const INVERSE_256 : Elem = Elem :: new (8_347_681) ; let mut w = self . 0 . clone () ; let mut m = 256 ; for len in [1 , 2 , 4 , 8 , 16 , 32 , 64 , 128] { for start in (0 .. 256) . step_by (2 * len) { m -= 1 ; let z = - ZETA_POW_BITREV [m] ; for j in start .. (start + len) { let t = w [j] ; w [j] = t + w [j + len] ; w [j + len] = z * (t - w [j + len]) ; } } } INVERSE_256 * & Polynomial :: new (w) } }
};
}
