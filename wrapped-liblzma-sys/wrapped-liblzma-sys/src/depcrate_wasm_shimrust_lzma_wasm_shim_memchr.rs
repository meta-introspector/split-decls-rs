// Generated macro for rust_lzma_wasm_shim_memchr (function)
macro_rules! Depcrate_wasm_shimrust_lzma_wasm_shim_memchr {
() => {
// Module: crate::wasm_shim
// Provides: {"rust_lzma_wasm_shim_memchr"}
// Dependencies: {}
# [no_mangle] pub unsafe extern "C" fn rust_lzma_wasm_shim_memchr (s : * const c_void , c : c_int , n : usize ,) -> * mut c_void { let s_slice = unsafe { core :: slice :: from_raw_parts (s as * const u8 , n) } ; s_slice . iter () . position (| & r | r == c as u8) . map_or (core :: ptr :: null_mut () , | p | unsafe { s . add (p) as * mut c_void }) }
};
}
