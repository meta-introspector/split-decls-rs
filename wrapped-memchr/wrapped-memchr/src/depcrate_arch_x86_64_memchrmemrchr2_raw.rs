// Generated macro for memrchr2_raw (function)
macro_rules! Depcrate_arch_x86_64_memchrmemrchr2_raw {
() => {
// Module: crate::arch::x86_64::memchr
// Provides: {"memrchr2_raw"}
// Dependencies: {}
# [doc = " memrchr2, but using raw pointers to represent the haystack."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Pointers must be valid. See `Two::rfind_raw`."] # [inline (always)] pub (crate) fn memrchr2_raw (n1 : u8 , n2 : u8 , start : * const u8 , end : * const u8 ,) -> Option < * const u8 > { unsafe_ifunc ! (Two , rfind_raw , unsafe fn (u8 , u8 , * const u8 , * const u8) -> Option <* const u8 >, Option <* const u8 >, start , end , n1 , n2) }
};
}
