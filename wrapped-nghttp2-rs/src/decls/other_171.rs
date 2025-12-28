macro_rules! other_171 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked when library tells error message to"] # [doc = " the application."] # [doc = ""] # [doc = " This function is deprecated.  The new application should use"] # [doc = " `nghttp2_session_callbacks_set_error_callback2()`."] # [doc = ""] # [doc = " If both :type:`nghttp2_error_callback` and"] # [doc = " :type:`nghttp2_error_callback2` are set, the latter takes"] # [doc = " precedence."] pub fn nghttp2_session_callbacks_set_error_callback (cbs : * mut nghttp2_session_callbacks , error_callback : nghttp2_error_callback ,) ; }
    };
}

other_171!()