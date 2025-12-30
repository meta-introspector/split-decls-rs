// Generated macro for other_156 (other)
macro_rules! Depcrateother_156 {
() => {
// Module: crate
// Provides: {"other_156"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked before a non-DATA frame is sent."] pub fn nghttp2_session_callbacks_set_before_frame_send_callback (cbs : * mut nghttp2_session_callbacks , before_frame_send_callback : nghttp2_before_frame_send_callback ,) ; }
};
}
