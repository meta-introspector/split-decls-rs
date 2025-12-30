// Generated macro for memrchr3_raw (function)
macro_rules! Depcrate_arch_aarch64_memchrmemrchr3_raw {
() => {
// Module: crate::arch::aarch64::memchr
// Provides: {"memrchr3_raw"}
// Dependencies: {}
# [doc = " memrchr3, but using raw pointers to represent the haystack."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Pointers must be valid. See `Three::rfind_raw`."] # [inline (always)] pub (crate) unsafe fn memrchr3_raw (n1 : u8 , n2 : u8 , n3 : u8 , start : * const u8 , end : * const u8 ,) -> Option < * const u8 > { defraw ! (Three , rfind_raw , start , end , n1 , n2 , n3) }
};
}
