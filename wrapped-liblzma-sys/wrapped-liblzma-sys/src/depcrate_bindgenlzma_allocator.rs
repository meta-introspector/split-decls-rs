// Generated macro for lzma_allocator (struct)
macro_rules! Depcrate_bindgenlzma_allocator {
() => {
// Module: crate::bindgen
// Provides: {"lzma_allocator"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Copy , Clone)] pub struct lzma_allocator { pub alloc : :: std :: option :: Option < unsafe extern "C" fn (opaque : * mut :: std :: os :: raw :: c_void , nmemb : usize , size : usize ,) -> * mut :: std :: os :: raw :: c_void , > , pub free : :: std :: option :: Option < unsafe extern "C" fn (opaque : * mut :: std :: os :: raw :: c_void , ptr : * mut :: std :: os :: raw :: c_void) , > , pub opaque : * mut :: std :: os :: raw :: c_void , }
};
}
