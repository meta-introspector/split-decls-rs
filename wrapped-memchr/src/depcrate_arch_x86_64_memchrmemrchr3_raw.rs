// Generated macro for memrchr3_raw (function)
macro_rules! Depcrate_arch_x86_64_memchrmemrchr3_raw {
() => {
// Module: crate::arch::x86_64::memchr
// Provides: {"memrchr3_raw"}
// Dependencies: {}
# [doc = " memrchr3, but using raw pointers to represent the haystack."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Pointers must be valid. See `Three::rfind_raw`."] # [inline (always)] pub (crate) fn memrchr3_raw (n1 : u8 , n2 : u8 , n3 : u8 , start : * const u8 , end : * const u8 ,) -> Option < * const u8 > { unsafe_ifunc ! (Three , rfind_raw , unsafe fn (u8 , u8 , u8 , * const u8 , * const u8) -> Option <* const u8 >, Option <* const u8 >, start , end , n1 , n2 , n3) }
};
}
