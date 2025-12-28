macro_rules! other_162 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked when a invalid header name/value"] # [doc = " pair is received.  If both"] # [doc = " `nghttp2_session_callbacks_set_on_invalid_header_callback()` and"] # [doc = " `nghttp2_session_callbacks_set_on_invalid_header_callback2()` are"] # [doc = " used to set callbacks, the latter takes the precedence."] pub fn nghttp2_session_callbacks_set_on_invalid_header_callback (cbs : * mut nghttp2_session_callbacks , on_invalid_header_callback : nghttp2_on_invalid_header_callback ,) ; }
    };
}

other_162!()