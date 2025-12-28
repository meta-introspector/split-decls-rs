macro_rules! other_255 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns nonzero if |session| is initialized as server side session."] pub fn nghttp2_session_check_server_session (session : * mut nghttp2_session ,) -> :: std :: os :: raw :: c_int ; }
    };
}

other_255!()