// Generated macro for other_157 (other)
macro_rules! Depcrateother_157 {
() => {
// Module: crate
// Provides: {"other_157"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked after a frame is sent."] pub fn nghttp2_session_callbacks_set_on_frame_send_callback (cbs : * mut nghttp2_session_callbacks , on_frame_send_callback : nghttp2_on_frame_send_callback ,) ; }
};
}
