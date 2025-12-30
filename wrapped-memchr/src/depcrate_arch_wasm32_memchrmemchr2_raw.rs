// Generated macro for memchr2_raw (function)
macro_rules! Depcrate_arch_wasm32_memchrmemchr2_raw {
() => {
// Module: crate::arch::wasm32::memchr
// Provides: {"memchr2_raw"}
// Dependencies: {}
# [doc = " memchr2, but using raw pointers to represent the haystack."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Pointers must be valid. See `Two::find_raw`."] # [inline (always)] pub (crate) unsafe fn memchr2_raw (n1 : u8 , n2 : u8 , start : * const u8 , end : * const u8 ,) -> Option < * const u8 > { defraw ! (Two , find_raw , start , end , n1 , n2) }
};
}
