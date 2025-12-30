// Generated macro for nghttp2_malloc (type)
macro_rules! Depcratenghttp2_malloc {
() => {
// Module: crate
// Provides: {"nghttp2_malloc"}
// Dependencies: {}
# [doc = " @functypedef"] # [doc = ""] # [doc = " Custom memory allocator to replace malloc().  The |mem_user_data|"] # [doc = " is the mem_user_data member of :type:`nghttp2_mem` structure."] pub type nghttp2_malloc = :: std :: option :: Option < unsafe extern "C" fn (size : usize , mem_user_data : * mut :: std :: os :: raw :: c_void ,) -> * mut :: std :: os :: raw :: c_void , > ;
};
}
