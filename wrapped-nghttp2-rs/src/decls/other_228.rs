macro_rules! other_228 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the next outgoing stream ID.  Notice that return type is"] # [doc = " uint32_t.  If we run out of stream ID for this session, this"] # [doc = " function returns 1 << 31."] pub fn nghttp2_session_get_next_stream_id (session : * mut nghttp2_session) -> u32 ; }
    };
}

other_228!()