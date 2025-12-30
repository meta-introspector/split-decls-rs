// Generated macro for memrchr_raw (function)
macro_rules! Depcrate_arch_x86_64_memchrmemrchr_raw {
() => {
// Module: crate::arch::x86_64::memchr
// Provides: {"memrchr_raw"}
// Dependencies: {}
# [doc = " memrchr, but using raw pointers to represent the haystack."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Pointers must be valid. See `One::rfind_raw`."] # [inline (always)] pub (crate) fn memrchr_raw (n1 : u8 , start : * const u8 , end : * const u8 ,) -> Option < * const u8 > { unsafe_ifunc ! (One , rfind_raw , unsafe fn (u8 , * const u8 , * const u8) -> Option <* const u8 >, Option <* const u8 >, start , end , n1) }
};
}
