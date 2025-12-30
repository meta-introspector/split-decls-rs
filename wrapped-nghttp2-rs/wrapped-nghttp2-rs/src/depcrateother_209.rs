// Generated macro for other_209 (other)
macro_rules! Depcrateother_209 {
() => {
// Module: crate
// Provides: {"other_209"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets |user_data| to |session|, overwriting the existing user data"] # [doc = " specified in `nghttp2_session_client_new()`, or"] # [doc = " `nghttp2_session_server_new()`."] pub fn nghttp2_session_set_user_data (session : * mut nghttp2_session , user_data : * mut :: std :: os :: raw :: c_void ,) ; }
};
}
