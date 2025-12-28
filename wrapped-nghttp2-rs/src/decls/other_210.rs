macro_rules! other_210 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the number of DATA payload in bytes received without"] # [doc = " WINDOW_UPDATE transmission for the stream |stream_id|.  The local"] # [doc = " (receive) window size can be adjusted by"] # [doc = " `nghttp2_submit_window_update()`.  This function takes into account"] # [doc = " that and returns effective data length.  In particular, if the"] # [doc = " local window size is reduced by submitting negative"] # [doc = " window_size_increment with `nghttp2_submit_window_update()`, this"] # [doc = " function returns the number of bytes less than actually received."] # [doc = ""] # [doc = " This function returns -1 if it fails."] pub fn nghttp2_session_get_stream_effective_recv_data_length (session : * mut nghttp2_session , stream_id : i32 ,) -> i32 ; }
    };
}

other_210!();