// Generated macro for lower_n_halfway (function)
macro_rules! Depcrate_lexical_roundinglower_n_halfway {
() => {
// Module: crate::lexical::rounding
// Provides: {"lower_n_halfway"}
// Dependencies: {}
# [doc = " Calculate the halfway point for the lower `n` bits."] # [inline] pub (crate) fn lower_n_halfway (n : u64) -> u64 { let bits : u64 = mem :: size_of :: < u64 > () as u64 * 8 ; debug_assert ! (n <= bits , "lower_n_halfway() overflow in shl.") ; if n == 0 { 0 } else { nth_bit (n - 1) } }
};
}
