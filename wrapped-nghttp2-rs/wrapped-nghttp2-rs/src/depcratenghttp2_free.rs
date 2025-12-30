// Generated macro for nghttp2_free (type)
macro_rules! Depcratenghttp2_free {
() => {
// Module: crate
// Provides: {"nghttp2_free"}
// Dependencies: {}
# [doc = " @functypedef"] # [doc = ""] # [doc = " Custom memory allocator to replace free().  The |mem_user_data| is"] # [doc = " the mem_user_data member of :type:`nghttp2_mem` structure."] pub type nghttp2_free = :: std :: option :: Option < unsafe extern "C" fn (ptr : * mut :: std :: os :: raw :: c_void , mem_user_data : * mut :: std :: os :: raw :: c_void ,) , > ;
};
}
