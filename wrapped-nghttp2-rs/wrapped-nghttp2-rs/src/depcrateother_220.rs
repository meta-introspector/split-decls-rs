// Generated macro for other_220 (other)
macro_rules! Depcrateother_220 {
() => {
// Module: crate
// Provides: {"other_220"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns 1 if remote peer half closed the given stream |stream_id|."] # [doc = " Returns 0 if it did not.  Returns -1 if no such stream exists."] pub fn nghttp2_session_get_stream_remote_close (session : * mut nghttp2_session , stream_id : i32 ,) -> :: std :: os :: raw :: c_int ; }
};
}
