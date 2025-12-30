// Generated macro for other_167 (other)
macro_rules! Depcrateother_167 {
() => {
// Module: crate
// Provides: {"other_167"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked when a frame header is received."] pub fn nghttp2_session_callbacks_set_on_begin_frame_callback (cbs : * mut nghttp2_session_callbacks , on_begin_frame_callback : nghttp2_on_begin_frame_callback ,) ; }
};
}
