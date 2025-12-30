// Generated macro for lower_n_mask (function)
macro_rules! Depcrate_lexical_roundinglower_n_mask {
() => {
// Module: crate::lexical::rounding
// Provides: {"lower_n_mask"}
// Dependencies: {}
# [doc = " Generate a bitwise mask for the lower `n` bits."] # [inline] pub (crate) fn lower_n_mask (n : u64) -> u64 { let bits : u64 = mem :: size_of :: < u64 > () as u64 * 8 ; debug_assert ! (n <= bits , "lower_n_mask() overflow in shl.") ; if n == bits { u64 :: MAX } else { (1 << n) - 1 } }
};
}
