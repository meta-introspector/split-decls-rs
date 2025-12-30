// Generated macro for other_154 (other)
macro_rules! Depcrateother_154 {
() => {
// Module: crate
// Provides: {"other_154"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked by `nghttp2_session_recv()` and"] # [doc = " `nghttp2_session_mem_recv()` when an invalid non-DATA frame is"] # [doc = " received."] pub fn nghttp2_session_callbacks_set_on_invalid_frame_recv_callback (cbs : * mut nghttp2_session_callbacks , on_invalid_frame_recv_callback : nghttp2_on_invalid_frame_recv_callback ,) ; }
};
}
