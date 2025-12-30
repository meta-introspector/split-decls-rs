// Generated macro for count_raw (function)
macro_rules! Depcrate_arch_aarch64_memchrcount_raw {
() => {
// Module: crate::arch::aarch64::memchr
// Provides: {"count_raw"}
// Dependencies: {}
# [doc = " Count all matching bytes, but using raw pointers to represent the haystack."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Pointers must be valid. See `One::count_raw`."] # [inline (always)] pub (crate) unsafe fn count_raw (n1 : u8 , start : * const u8 , end : * const u8 ,) -> usize { defraw ! (One , count_raw , start , end , n1) }
};
}
