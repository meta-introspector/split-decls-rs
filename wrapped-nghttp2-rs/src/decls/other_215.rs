macro_rules! other_215 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the amount of flow-controlled payload (e.g., DATA) that the"] # [doc = " remote endpoint can send without receiving connection level"] # [doc = " WINDOW_UPDATE frame.  Note that each stream is still subject to the"] # [doc = " stream level flow control (see"] # [doc = " `nghttp2_session_get_stream_local_window_size()`)."] # [doc = ""] # [doc = " This function returns -1 if it fails."] pub fn nghttp2_session_get_local_window_size (session : * mut nghttp2_session) -> i32 ; }
    };
}

other_215!()