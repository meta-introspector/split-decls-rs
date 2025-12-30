// Generated macro for other_219 (other)
macro_rules! Depcrateother_219 {
() => {
// Module: crate
// Provides: {"other_219"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns 1 if local peer half closed the given stream |stream_id|."] # [doc = " Returns 0 if it did not.  Returns -1 if no such stream exists."] pub fn nghttp2_session_get_stream_local_close (session : * mut nghttp2_session , stream_id : i32 ,) -> :: std :: os :: raw :: c_int ; }
};
}
