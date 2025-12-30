// Generated macro for multiply (function)
macro_rules! Depcrate_combinemultiply {
() => {
// Module: crate::combine
// Provides: {"multiply"}
// Dependencies: {}
fn multiply (a : u32 , mut b : u32) -> u32 { let mut p = 0u32 ; for i in 0 .. 32 { p ^= b & ((a >> (31 - i)) & 1) . wrapping_neg () ; b = (b >> 1) ^ ((b & 1) . wrapping_neg () & POLY) ; } p }
};
}
