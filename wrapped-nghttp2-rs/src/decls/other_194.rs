macro_rules! other_194 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Like `nghttp2_session_client_new()`, but with additional options"] # [doc = " specified in the |option|."] # [doc = ""] # [doc = " The |option| can be ``NULL`` and the call is equivalent to"] # [doc = " `nghttp2_session_client_new()`."] # [doc = ""] # [doc = " This function does not take ownership |option|.  The application is"] # [doc = " responsible for freeing |option| if it finishes using the object."] # [doc = ""] # [doc = " The library code does not refer to |option| after this function"] # [doc = " returns."] # [doc = ""] # [doc = " This function returns 0 if it succeeds, or one of the following"] # [doc = " negative error codes:"] # [doc = ""] # [doc = " :enum:`NGHTTP2_ERR_NOMEM`"] # [doc = "     Out of memory."] pub fn nghttp2_session_client_new2 (session_ptr : * mut * mut nghttp2_session , callbacks : * const nghttp2_session_callbacks , user_data : * mut :: std :: os :: raw :: c_void , option : * const nghttp2_option ,) -> :: std :: os :: raw :: c_int ; }
    };
}

other_194!();