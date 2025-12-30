// Generated macro for other_232 (other)
macro_rules! Depcrateother_232 {
() => {
// Module: crate
// Provides: {"other_232"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Like `nghttp2_session_consume()`, but this only tells library that"] # [doc = " |size| bytes were consumed only for stream denoted by |stream_id|."] # [doc = " Note that HTTP/2 maintains connection and stream level flow control"] # [doc = " windows independently."] # [doc = ""] # [doc = " This function returns 0 if it succeeds, or one of the following"] # [doc = " negative error codes:"] # [doc = ""] # [doc = " :enum:`NGHTTP2_ERR_NOMEM`"] # [doc = "     Out of memory."] # [doc = " :enum:`NGHTTP2_ERR_INVALID_ARGUMENT`"] # [doc = "     The |stream_id| is 0."] # [doc = " :enum:`NGHTTP2_ERR_INVALID_STATE`"] # [doc = "     Automatic WINDOW_UPDATE is not disabled."] pub fn nghttp2_session_consume_stream (session : * mut nghttp2_session , stream_id : i32 , size : usize ,) -> :: std :: os :: raw :: c_int ; }
};
}
