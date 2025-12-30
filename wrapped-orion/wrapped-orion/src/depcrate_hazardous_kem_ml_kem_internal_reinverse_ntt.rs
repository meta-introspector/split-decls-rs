// Generated macro for inverse_ntt (function)
macro_rules! Depcrate_hazardous_kem_ml_kem_internal_reinverse_ntt {
() => {
// Module: crate::hazardous::kem::ml_kem::internal::re
// Provides: {"inverse_ntt"}
// Dependencies: {}
pub fn inverse_ntt (f_hat : & RingElementNTT) -> RingElement { let mut f = RingElement :: copy_from_ntt (f_hat) ; let mut len = 2 ; let mut i = 127 ; while len <= 128 { let mut start = 0 ; while start < 256 { let zeta = FieldElement :: new (ZETA_ALL [i]) ; i -= 1 ; for j in start .. (start + len) { let t : FieldElement = f [j] ; f [j] = t + f [j + len] ; f [j + len] = zeta * (f [j + len] - t) ; } start += 2 * len ; } len *= 2 ; } for fe in f . coefficients . iter_mut () { * fe = * fe * FieldElement (3303) ; } f }
};
}
