// Generated macro for u64_to_hi64_1 (function)
macro_rules! Depcrate_lexical_mathu64_to_hi64_1 {
() => {
// Module: crate::lexical::math
// Provides: {"u64_to_hi64_1"}
// Dependencies: {}
# [doc = " Shift 64-bit integer to high 64-bits."] # [inline] fn u64_to_hi64_1 (r0 : u64) -> (u64 , bool) { debug_assert ! (r0 != 0) ; let ls = r0 . leading_zeros () ; (r0 << ls , false) }
};
}
