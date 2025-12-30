// Generated macro for other_160 (other)
macro_rules! Depcrateother_160 {
() => {
// Module: crate
// Provides: {"other_160"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked when the reception of header block"] # [doc = " in HEADERS or PUSH_PROMISE is started."] pub fn nghttp2_session_callbacks_set_on_begin_headers_callback (cbs : * mut nghttp2_session_callbacks , on_begin_headers_callback : nghttp2_on_begin_headers_callback ,) ; }
};
}
