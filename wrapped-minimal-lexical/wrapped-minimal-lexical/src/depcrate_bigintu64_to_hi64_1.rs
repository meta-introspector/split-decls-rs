// Generated macro for u64_to_hi64_1 (function)
macro_rules! Depcrate_bigintu64_to_hi64_1 {
() => {
// Module: crate::bigint
// Provides: {"u64_to_hi64_1"}
// Dependencies: {}
# [doc = " Shift 64-bit integer to high 64-bits."] # [inline] pub fn u64_to_hi64_1 (r0 : u64) -> (u64 , bool) { let ls = r0 . leading_zeros () ; (r0 << ls , false) }
};
}
