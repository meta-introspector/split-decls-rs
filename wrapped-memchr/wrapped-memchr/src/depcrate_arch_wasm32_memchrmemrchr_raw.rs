// Generated macro for memrchr_raw (function)
macro_rules! Depcrate_arch_wasm32_memchrmemrchr_raw {
() => {
// Module: crate::arch::wasm32::memchr
// Provides: {"memrchr_raw"}
// Dependencies: {}
# [doc = " memrchr, but using raw pointers to represent the haystack."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Pointers must be valid. See `One::rfind_raw`."] # [inline (always)] pub (crate) unsafe fn memrchr_raw (n1 : u8 , start : * const u8 , end : * const u8 ,) -> Option < * const u8 > { defraw ! (One , rfind_raw , start , end , n1) }
};
}
