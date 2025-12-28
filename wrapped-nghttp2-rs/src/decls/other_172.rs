macro_rules! other_172 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked when library tells error code, and"] # [doc = " message to the application."] # [doc = ""] # [doc = " If both :type:`nghttp2_error_callback` and"] # [doc = " :type:`nghttp2_error_callback2` are set, the latter takes"] # [doc = " precedence."] pub fn nghttp2_session_callbacks_set_error_callback2 (cbs : * mut nghttp2_session_callbacks , error_callback2 : nghttp2_error_callback2 ,) ; }
    };
}

other_172!()