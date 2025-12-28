macro_rules! other_158 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked when the stream is closed."] pub fn nghttp2_session_callbacks_set_on_stream_close_callback (cbs : * mut nghttp2_session_callbacks , on_stream_close_callback : nghttp2_on_stream_close_callback ,) ; }
    };
}

other_158!()