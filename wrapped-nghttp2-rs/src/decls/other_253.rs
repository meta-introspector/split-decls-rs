macro_rules! other_253 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the last stream ID of a stream for which"] # [doc = " :type:`nghttp2_on_frame_recv_callback` was invoked most recently."] # [doc = " The returned value can be used as last_stream_id parameter for"] # [doc = " `nghttp2_submit_goaway()` and"] # [doc = " `nghttp2_session_terminate_session2()`."] # [doc = ""] # [doc = " This function always succeeds."] pub fn nghttp2_session_get_last_proc_stream_id (session : * mut nghttp2_session) -> i32 ; }
    };
}

other_253!();