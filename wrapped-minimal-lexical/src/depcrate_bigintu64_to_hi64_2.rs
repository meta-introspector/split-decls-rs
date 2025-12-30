// Generated macro for u64_to_hi64_2 (function)
macro_rules! Depcrate_bigintu64_to_hi64_2 {
() => {
// Module: crate::bigint
// Provides: {"u64_to_hi64_2"}
// Dependencies: {}
# [doc = " Shift 2 64-bit integers to high 64-bits."] # [inline] pub fn u64_to_hi64_2 (r0 : u64 , r1 : u64) -> (u64 , bool) { let ls = r0 . leading_zeros () ; let rs = 64 - ls ; let v = match ls { 0 => r0 , _ => (r0 << ls) | (r1 >> rs) , } ; let n = r1 << ls != 0 ; (v , n) }
};
}
