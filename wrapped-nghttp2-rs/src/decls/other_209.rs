macro_rules! other_209 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the number of frames in the outbound queue.  This does not"] # [doc = " include the deferred DATA frames."] pub fn nghttp2_session_get_outbound_queue_size (session : * mut nghttp2_session) -> usize ; }
    };
}

other_209!()