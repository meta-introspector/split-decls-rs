macro_rules! other_214 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the local (receive) window size for a connection.  The"] # [doc = " local window size can be adjusted by"] # [doc = " `nghttp2_submit_window_update()`.  This function takes into account"] # [doc = " that and returns effective window size."] # [doc = ""] # [doc = " This function does not take into account the amount of received"] # [doc = " data from the remote endpoint.  Use"] # [doc = " `nghttp2_session_get_local_window_size()` to know the amount of"] # [doc = " data the remote endpoint can send without receiving"] # [doc = " connection-level WINDOW_UPDATE frame.  Note that each stream is"] # [doc = " still subject to the stream level flow control."] # [doc = ""] # [doc = " This function returns -1 if it fails."] pub fn nghttp2_session_get_effective_local_window_size (session : * mut nghttp2_session) -> i32 ; }
    };
}

other_214!()