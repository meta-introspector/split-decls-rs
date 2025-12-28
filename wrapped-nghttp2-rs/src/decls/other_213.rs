macro_rules! other_213 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the number of DATA payload in bytes received without"] # [doc = " WINDOW_UPDATE transmission for a connection.  The local (receive)"] # [doc = " window size can be adjusted by `nghttp2_submit_window_update()`."] # [doc = " This function takes into account that and returns effective data"] # [doc = " length.  In particular, if the local window size is reduced by"] # [doc = " submitting negative window_size_increment with"] # [doc = " `nghttp2_submit_window_update()`, this function returns the number"] # [doc = " of bytes less than actually received."] # [doc = ""] # [doc = " This function returns -1 if it fails."] pub fn nghttp2_session_get_effective_recv_data_length (session : * mut nghttp2_session) -> i32 ; }
    };
}

other_213!();