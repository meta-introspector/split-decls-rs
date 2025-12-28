macro_rules! other_148 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Initializes |*callbacks_ptr| with NULL values."] # [doc = ""] # [doc = " The initialized object can be used when initializing multiple"] # [doc = " :type:`nghttp2_session` objects."] # [doc = ""] # [doc = " When the application finished using this object, it can use"] # [doc = " `nghttp2_session_callbacks_del()` to free its memory."] # [doc = ""] # [doc = " This function returns 0 if it succeeds, or one of the following"] # [doc = " negative error codes:"] # [doc = ""] # [doc = " :enum:`NGHTTP2_ERR_NOMEM`"] # [doc = "     Out of memory."] pub fn nghttp2_session_callbacks_new (callbacks_ptr : * mut * mut nghttp2_session_callbacks ,) -> :: std :: os :: raw :: c_int ; }
    };
}

other_148!()