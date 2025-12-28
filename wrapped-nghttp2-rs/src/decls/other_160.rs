macro_rules! other_160 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked when a header name/value pair is"] # [doc = " received.  If both"] # [doc = " `nghttp2_session_callbacks_set_on_header_callback()` and"] # [doc = " `nghttp2_session_callbacks_set_on_header_callback2()` are used to"] # [doc = " set callbacks, the latter has the precedence."] pub fn nghttp2_session_callbacks_set_on_header_callback (cbs : * mut nghttp2_session_callbacks , on_header_callback : nghttp2_on_header_callback ,) ; }
    };
}

other_160!()