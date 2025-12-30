// Generated macro for to_ntt (function)
macro_rules! Depcrate_hazardous_kem_ml_kem_internal_reto_ntt {
() => {
// Module: crate::hazardous::kem::ml_kem::internal::re
// Provides: {"to_ntt"}
// Dependencies: {}
pub fn to_ntt (f : & RingElement) -> RingElementNTT { let mut i = 1 ; let mut len = 128 ; let mut f_hat = RingElementNTT :: copy_from_non_ntt (f) ; while len >= 2 { let mut start = 0 ; while start < 256 { let zeta = FieldElement :: new (ZETA_ALL [i]) ; i += 1 ; for j in start .. (start + len) { let t : FieldElement = zeta * f_hat [j + len] ; f_hat [j + len] = f_hat [j] - t ; f_hat [j] = f_hat [j] + t ; } start += 2 * len ; } len >>= 1 ; } f_hat }
};
}
