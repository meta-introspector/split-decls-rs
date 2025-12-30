// Generated macro for wasmtime_memory_image_free (function)
macro_rules! Depcrate_capiwasmtime_memory_image_free {
() => {
// Module: crate::capi
// Provides: {"wasmtime_memory_image_free"}
// Dependencies: {}
# [doc = " Deallocates the provided `wasmtime_memory_image`."] # [doc = ""] # [doc = " Note that mappings created from this image are not guaranteed to be"] # [doc = " deallocated and/or unmapped before this is called."] # [unsafe (no_mangle)] pub extern "C" fn wasmtime_memory_image_free (_image : * mut wasmtime_memory_image) { error ! ("Currently. HermitOS doesn't support wasmtime_memory_image_free!") ; }
};
}
