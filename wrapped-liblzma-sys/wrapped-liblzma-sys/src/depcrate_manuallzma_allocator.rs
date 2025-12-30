// Generated macro for lzma_allocator (struct)
macro_rules! Depcrate_manuallzma_allocator {
() => {
// Module: crate::manual
// Provides: {"lzma_allocator"}
// Dependencies: {}
# [repr (C)] pub struct lzma_allocator { pub alloc : Option < extern "C" fn (* mut c_void , size_t , size_t) -> * mut c_void > , pub free : Option < extern "C" fn (* mut c_void , * mut c_void) > , pub opaque : * mut c_void , }
};
}
