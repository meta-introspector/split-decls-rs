// Generated macro for memchr_raw (function)
macro_rules! Depcrate_arch_aarch64_memchrmemchr_raw {
() => {
// Module: crate::arch::aarch64::memchr
// Provides: {"memchr_raw"}
// Dependencies: {}
# [doc = " memchr, but using raw pointers to represent the haystack."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Pointers must be valid. See `One::find_raw`."] # [inline (always)] pub (crate) unsafe fn memchr_raw (n1 : u8 , start : * const u8 , end : * const u8 ,) -> Option < * const u8 > { defraw ! (One , find_raw , start , end , n1) }
};
}
