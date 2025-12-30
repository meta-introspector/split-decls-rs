// Generated macro for memchr_raw (function)
macro_rules! Depcrate_arch_x86_64_memchrmemchr_raw {
() => {
// Module: crate::arch::x86_64::memchr
// Provides: {"memchr_raw"}
// Dependencies: {}
# [doc = " memchr, but using raw pointers to represent the haystack."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Pointers must be valid. See `One::find_raw`."] # [inline (always)] pub (crate) fn memchr_raw (n1 : u8 , start : * const u8 , end : * const u8 ,) -> Option < * const u8 > { unsafe_ifunc ! (One , find_raw , unsafe fn (u8 , * const u8 , * const u8) -> Option <* const u8 >, Option <* const u8 >, start , end , n1) }
};
}
