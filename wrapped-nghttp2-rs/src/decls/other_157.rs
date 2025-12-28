macro_rules! other_157 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked when a non-DATA frame is not sent"] # [doc = " because of an error."] pub fn nghttp2_session_callbacks_set_on_frame_not_send_callback (cbs : * mut nghttp2_session_callbacks , on_frame_not_send_callback : nghttp2_on_frame_not_send_callback ,) ; }
    };
}

other_157!();