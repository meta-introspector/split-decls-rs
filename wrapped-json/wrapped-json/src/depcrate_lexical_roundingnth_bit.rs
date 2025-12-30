// Generated macro for nth_bit (function)
macro_rules! Depcrate_lexical_roundingnth_bit {
() => {
// Module: crate::lexical::rounding
// Provides: {"nth_bit"}
// Dependencies: {}
# [doc = " Calculate a scalar factor of 2 above the halfway point."] # [inline] pub (crate) fn nth_bit (n : u64) -> u64 { let bits : u64 = mem :: size_of :: < u64 > () as u64 * 8 ; debug_assert ! (n < bits , "nth_bit() overflow in shl.") ; 1 << n }
};
}
