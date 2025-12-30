// Generated macro for copysign32 (function)
macro_rules! Depcrate_util_libmcopysign32 {
() => {
// Module: crate::util::libm
// Provides: {"copysign32"}
// Dependencies: {}
fn copysign32 (x : f32 , y : f32) -> f32 { let mut ux = x . to_bits () ; let uy = y . to_bits () ; ux &= 0x7fffffff ; ux |= uy & 0x80000000 ; f32 :: from_bits (ux) }
};
}
