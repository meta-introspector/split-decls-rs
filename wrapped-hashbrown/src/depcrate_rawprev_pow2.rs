// Generated macro for prev_pow2 (function)
macro_rules! Depcrate_rawprev_pow2 {
() => {
// Module: crate::raw
// Provides: {"prev_pow2"}
// Dependencies: {}
# [doc = " Find the previous power of 2. If it's already a power of 2, it's unchanged."] # [doc = " Passing zero is undefined behavior."] pub (crate) fn prev_pow2 (z : usize) -> usize { let shift = mem :: size_of :: < usize > () * 8 - 1 ; 1 << (shift - (z . leading_zeros () as usize)) }
};
}
