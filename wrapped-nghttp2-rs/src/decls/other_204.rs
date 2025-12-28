macro_rules! other_204 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns nonzero value if |session| wants to receive data from the"] # [doc = " remote peer."] # [doc = ""] # [doc = " If both `nghttp2_session_want_read()` and"] # [doc = " `nghttp2_session_want_write()` return 0, the application should"] # [doc = " drop the connection."] pub fn nghttp2_session_want_read (session : * mut nghttp2_session) -> :: std :: os :: raw :: c_int ; }
    };
}

other_204!();