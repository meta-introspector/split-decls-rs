macro_rules! other_205 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns nonzero value if |session| wants to send data to the remote"] # [doc = " peer."] # [doc = ""] # [doc = " If both `nghttp2_session_want_read()` and"] # [doc = " `nghttp2_session_want_write()` return 0, the application should"] # [doc = " drop the connection."] pub fn nghttp2_session_want_write (session : * mut nghttp2_session) -> :: std :: os :: raw :: c_int ; }
    };
}

other_205!();