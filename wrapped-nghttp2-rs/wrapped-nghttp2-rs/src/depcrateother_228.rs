// Generated macro for other_228 (other)
macro_rules! Depcrateother_228 {
() => {
// Module: crate
// Provides: {"other_228"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Tells the |session| that next stream ID is |next_stream_id|.  The"] # [doc = " |next_stream_id| must be equal or greater than the value returned"] # [doc = " by `nghttp2_session_get_next_stream_id()`."] # [doc = ""] # [doc = " This function returns 0 if it succeeds, or one of the following"] # [doc = " negative error codes:"] # [doc = ""] # [doc = " :enum:`NGHTTP2_ERR_INVALID_ARGUMENT`"] # [doc = "     The |next_stream_id| is strictly less than the value"] # [doc = "     `nghttp2_session_get_next_stream_id()` returns; or"] # [doc = "     |next_stream_id| is invalid (e.g., even integer for client, or"] # [doc = "     odd integer for server)."] pub fn nghttp2_session_set_next_stream_id (session : * mut nghttp2_session , next_stream_id : i32 ,) -> :: std :: os :: raw :: c_int ; }
};
}
