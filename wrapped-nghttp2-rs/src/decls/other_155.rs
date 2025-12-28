macro_rules! other_155 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked before a non-DATA frame is sent."] pub fn nghttp2_session_callbacks_set_before_frame_send_callback (cbs : * mut nghttp2_session_callbacks , before_frame_send_callback : nghttp2_before_frame_send_callback ,) ; }
    };
}

other_155!()