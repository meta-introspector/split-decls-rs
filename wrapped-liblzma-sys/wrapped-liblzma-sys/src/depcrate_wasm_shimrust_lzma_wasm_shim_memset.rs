// Generated macro for rust_lzma_wasm_shim_memset (function)
macro_rules! Depcrate_wasm_shimrust_lzma_wasm_shim_memset {
() => {
// Module: crate::wasm_shim
// Provides: {"rust_lzma_wasm_shim_memset"}
// Dependencies: {}
# [no_mangle] pub unsafe extern "C" fn rust_lzma_wasm_shim_memset (dest : * mut c_void , c : c_int , n : usize ,) -> * mut c_void { core :: ptr :: write_bytes (dest as * mut u8 , c as u8 , n) ; dest }
};
}
