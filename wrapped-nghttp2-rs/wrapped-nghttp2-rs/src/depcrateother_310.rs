// Generated macro for other_310 (other)
macro_rules! Depcrateother_310 {
() => {
// Module: crate
// Provides: {"other_310"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns root of dependency tree, which is imaginary stream with"] # [doc = " stream ID 0.  The returned pointer is valid until |session| is"] # [doc = " freed by `nghttp2_session_del()`."] pub fn nghttp2_session_get_root_stream (session : * mut nghttp2_session) -> * mut nghttp2_stream ; }
};
}
