// Generated macro for rust_lzma_wasm_shim_memcmp (function)
macro_rules! Depcrate_wasm_shimrust_lzma_wasm_shim_memcmp {
() => {
// Module: crate::wasm_shim
// Provides: {"rust_lzma_wasm_shim_memcmp"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn rust_lzma_wasm_shim_memcmp (str1 : * const c_void , str2 : * const c_void , n : usize ,) -> i32 { unsafe { let str1 : & [u8] = core :: slice :: from_raw_parts (str1 as * const u8 , n) ; let str2 : & [u8] = core :: slice :: from_raw_parts (str2 as * const u8 , n) ; match str1 . cmp (str2) { core :: cmp :: Ordering :: Less => - 1 , core :: cmp :: Ordering :: Equal => 0 , core :: cmp :: Ordering :: Greater => 1 , } } }
};
}
