// Generated macro for other_207 (other)
macro_rules! Depcrateother_207 {
() => {
// Module: crate
// Provides: {"other_207"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns stream_user_data for the stream |stream_id|.  The"] # [doc = " stream_user_data is provided by `nghttp2_submit_request()`,"] # [doc = " `nghttp2_submit_headers()` or"] # [doc = " `nghttp2_session_set_stream_user_data()`.  Unless it is set using"] # [doc = " `nghttp2_session_set_stream_user_data()`, if the stream is"] # [doc = " initiated by the remote endpoint, stream_user_data is always"] # [doc = " ``NULL``.  If the stream does not exist, this function returns"] # [doc = " ``NULL``."] pub fn nghttp2_session_get_stream_user_data (session : * mut nghttp2_session , stream_id : i32 ,) -> * mut :: std :: os :: raw :: c_void ; }
};
}
