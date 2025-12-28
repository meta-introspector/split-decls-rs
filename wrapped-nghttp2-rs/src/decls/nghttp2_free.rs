macro_rules! nghttp2_free {
    () => {
        # [doc = " @functypedef"] # [doc = ""] # [doc = " Custom memory allocator to replace free().  The |mem_user_data| is"] # [doc = " the mem_user_data member of :type:`nghttp2_mem` structure."] pub type nghttp2_free = :: std :: option :: Option < unsafe extern "C" fn (ptr : * mut :: std :: os :: raw :: c_void , mem_user_data : * mut :: std :: os :: raw :: c_void ,) , > ;
    };
}

nghttp2_free!()