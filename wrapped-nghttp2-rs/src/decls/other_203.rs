macro_rules! other_203 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Puts back previously deferred DATA frame in the stream |stream_id|"] # [doc = " to the outbound queue."] # [doc = ""] # [doc = " This function returns 0 if it succeeds, or one of the following"] # [doc = " negative error codes:"] # [doc = ""] # [doc = " :enum:`NGHTTP2_ERR_INVALID_ARGUMENT`"] # [doc = "     The stream does not exist; or no deferred data exist."] # [doc = " :enum:`NGHTTP2_ERR_NOMEM`"] # [doc = "     Out of memory."] pub fn nghttp2_session_resume_data (session : * mut nghttp2_session , stream_id : i32 ,) -> :: std :: os :: raw :: c_int ; }
    };
}

other_203!()