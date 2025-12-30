// Generated macro for one_per_byte (function)
macro_rules! Depcrate_intone_per_byte {
() => {
// Module: crate::int
// Provides: {"one_per_byte"}
// Dependencies: {}
fn one_per_byte < P : PrimInt > () -> P { let mut ret = P :: one () ; let mut shift = 8 ; let mut b = ret . count_zeros () >> 3 ; while b != 0 { ret = (ret << shift) | ret ; shift <<= 1 ; b >>= 1 ; } ret }
};
}
