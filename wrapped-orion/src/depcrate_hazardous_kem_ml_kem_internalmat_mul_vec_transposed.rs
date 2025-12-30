// Generated macro for mat_mul_vec_transposed (function)
macro_rules! Depcrate_hazardous_kem_ml_kem_internalmat_mul_vec_transposed {
() => {
// Module: crate::hazardous::kem::ml_kem::internal
// Provides: {"mat_mul_vec_transposed"}
// Dependencies: {}
pub fn mat_mul_vec_transposed < const K : usize > (mat : & [[RingElementNTT ; K]] , vec : & [RingElementNTT] ,) -> [RingElementNTT ; K] { let mut ret = [RingElementNTT :: zero () ; K] ; for (i , r) in ret . iter_mut () . enumerate () { for j in 0 .. K { let product = mat [j] [i] * vec [j] ; * r += product ; } } ret }
};
}
