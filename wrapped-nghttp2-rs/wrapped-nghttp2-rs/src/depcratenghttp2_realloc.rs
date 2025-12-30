// Generated macro for nghttp2_realloc (type)
macro_rules! Depcratenghttp2_realloc {
() => {
// Module: crate
// Provides: {"nghttp2_realloc"}
// Dependencies: {}
# [doc = " @functypedef"] # [doc = ""] # [doc = " Custom memory allocator to replace realloc().  The |mem_user_data|"] # [doc = " is the mem_user_data member of :type:`nghttp2_mem` structure."] pub type nghttp2_realloc = :: std :: option :: Option < unsafe extern "C" fn (ptr : * mut :: std :: os :: raw :: c_void , size : usize , mem_user_data : * mut :: std :: os :: raw :: c_void ,) -> * mut :: std :: os :: raw :: c_void , > ;
};
}
