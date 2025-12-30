// Generated macro for nghttp2_calloc (type)
macro_rules! Depcratenghttp2_calloc {
() => {
// Module: crate
// Provides: {"nghttp2_calloc"}
// Dependencies: {}
# [doc = " @functypedef"] # [doc = ""] # [doc = " Custom memory allocator to replace calloc().  The |mem_user_data|"] # [doc = " is the mem_user_data member of :type:`nghttp2_mem` structure."] pub type nghttp2_calloc = :: std :: option :: Option < unsafe extern "C" fn (nmemb : usize , size : usize , mem_user_data : * mut :: std :: os :: raw :: c_void ,) -> * mut :: std :: os :: raw :: c_void , > ;
};
}
