macro_rules! other_211 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the local (receive) window size for the stream |stream_id|."] # [doc = " The local window size can be adjusted by"] # [doc = " `nghttp2_submit_window_update()`.  This function takes into account"] # [doc = " that and returns effective window size."] # [doc = ""] # [doc = " This function does not take into account the amount of received"] # [doc = " data from the remote endpoint.  Use"] # [doc = " `nghttp2_session_get_stream_local_window_size()` to know the amount"] # [doc = " of data the remote endpoint can send without receiving stream level"] # [doc = " WINDOW_UPDATE frame.  Note that each stream is still subject to the"] # [doc = " connection level flow control."] # [doc = ""] # [doc = " This function returns -1 if it fails."] pub fn nghttp2_session_get_stream_effective_local_window_size (session : * mut nghttp2_session , stream_id : i32 ,) -> i32 ; }
    };
}

other_211!();