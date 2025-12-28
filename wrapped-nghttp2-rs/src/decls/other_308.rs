macro_rules! other_308 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns state of |stream|.  The root stream retrieved by"] # [doc = " `nghttp2_session_get_root_stream()` will have stream state"] # [doc = " :enum:`NGHTTP2_STREAM_STATE_IDLE`."] pub fn nghttp2_stream_get_state (stream : * mut nghttp2_stream) -> nghttp2_stream_proto_state ; }
    };
}

other_308!();