// Generated macro for copysign64 (function)
macro_rules! Depcrate_util_libmcopysign64 {
() => {
// Module: crate::util::libm
// Provides: {"copysign64"}
// Dependencies: {}
fn copysign64 (x : f64 , y : f64) -> f64 { let mut ux = x . to_bits () ; let uy = y . to_bits () ; ux &= (! 0) >> 1 ; ux |= uy & (1 << 63) ; f64 :: from_bits (ux) }
};
}
