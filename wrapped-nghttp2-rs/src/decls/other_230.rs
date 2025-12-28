macro_rules! other_230 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Like `nghttp2_session_consume()`, but this only tells library that"] # [doc = " |size| bytes were consumed only for connection level.  Note that"] # [doc = " HTTP/2 maintains connection and stream level flow control windows"] # [doc = " independently."] # [doc = ""] # [doc = " This function returns 0 if it succeeds, or one of the following"] # [doc = " negative error codes:"] # [doc = ""] # [doc = " :enum:`NGHTTP2_ERR_NOMEM`"] # [doc = "     Out of memory."] # [doc = " :enum:`NGHTTP2_ERR_INVALID_STATE`"] # [doc = "     Automatic WINDOW_UPDATE is not disabled."] pub fn nghttp2_session_consume_connection (session : * mut nghttp2_session , size : usize ,) -> :: std :: os :: raw :: c_int ; }
    };
}

other_230!()