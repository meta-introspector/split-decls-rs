// Generated macro for count_raw (function)
macro_rules! Depcrate_arch_x86_64_memchrcount_raw {
() => {
// Module: crate::arch::x86_64::memchr
// Provides: {"count_raw"}
// Dependencies: {}
# [doc = " Count all matching bytes, but using raw pointers to represent the haystack."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Pointers must be valid. See `One::count_raw`."] # [inline (always)] pub (crate) fn count_raw (n1 : u8 , start : * const u8 , end : * const u8) -> usize { unsafe_ifunc ! (One , count_raw , unsafe fn (u8 , * const u8 , * const u8) -> usize , usize , start , end , n1) }
};
}
