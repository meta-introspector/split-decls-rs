// Generated macro for other_164 (other)
macro_rules! Depcrateother_164 {
() => {
// Module: crate
// Provides: {"other_164"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked when a invalid header name/value"] # [doc = " pair is received."] pub fn nghttp2_session_callbacks_set_on_invalid_header_callback2 (cbs : * mut nghttp2_session_callbacks , on_invalid_header_callback2 : nghttp2_on_invalid_header_callback2 ,) ; }
};
}
