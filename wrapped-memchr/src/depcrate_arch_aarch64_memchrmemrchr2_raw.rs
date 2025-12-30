// Generated macro for memrchr2_raw (function)
macro_rules! Depcrate_arch_aarch64_memchrmemrchr2_raw {
() => {
// Module: crate::arch::aarch64::memchr
// Provides: {"memrchr2_raw"}
// Dependencies: {}
# [doc = " memrchr2, but using raw pointers to represent the haystack."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Pointers must be valid. See `Two::rfind_raw`."] # [inline (always)] pub (crate) unsafe fn memrchr2_raw (n1 : u8 , n2 : u8 , start : * const u8 , end : * const u8 ,) -> Option < * const u8 > { defraw ! (Two , rfind_raw , start , end , n1 , n2) }
};
}
