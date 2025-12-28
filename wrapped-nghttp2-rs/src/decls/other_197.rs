macro_rules! other_197 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Like `nghttp2_session_server_new2()`, but with additional custom"] # [doc = " memory allocator specified in the |mem|."] # [doc = ""] # [doc = " The |mem| can be ``NULL`` and the call is equivalent to"] # [doc = " `nghttp2_session_server_new2()`."] # [doc = ""] # [doc = " This function does not take ownership |mem|.  The application is"] # [doc = " responsible for freeing |mem|."] # [doc = ""] # [doc = " The library code does not refer to |mem| pointer after this"] # [doc = " function returns, so the application can safely free it."] # [doc = ""] # [doc = " This function returns 0 if it succeeds, or one of the following"] # [doc = " negative error codes:"] # [doc = ""] # [doc = " :enum:`NGHTTP2_ERR_NOMEM`"] # [doc = "     Out of memory."] pub fn nghttp2_session_server_new3 (session_ptr : * mut * mut nghttp2_session , callbacks : * const nghttp2_session_callbacks , user_data : * mut :: std :: os :: raw :: c_void , option : * const nghttp2_option , mem : * mut nghttp2_mem ,) -> :: std :: os :: raw :: c_int ; }
    };
}

other_197!();