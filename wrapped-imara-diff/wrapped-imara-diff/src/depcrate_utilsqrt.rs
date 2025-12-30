// Generated macro for sqrt (function)
macro_rules! Depcrate_utilsqrt {
() => {
// Module: crate::util
// Provides: {"sqrt"}
// Dependencies: {}
pub fn sqrt (val : usize) -> u32 { let nbits = (usize :: BITS - val . leading_zeros ()) / 2 ; 1 << nbits }
};
}
