macro_rules! other_152 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked by `nghttp2_session_recv()` and"] # [doc = " `nghttp2_session_mem_recv()` when a frame is received."] pub fn nghttp2_session_callbacks_set_on_frame_recv_callback (cbs : * mut nghttp2_session_callbacks , on_frame_recv_callback : nghttp2_on_frame_recv_callback ,) ; }
    };
}

other_152!();