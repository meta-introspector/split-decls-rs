// Generated macro for other_210 (other)
macro_rules! Depcrateother_210 {
() => {
// Module: crate
// Provides: {"other_210"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the number of frames in the outbound queue.  This does not"] # [doc = " include the deferred DATA frames."] pub fn nghttp2_session_get_outbound_queue_size (session : * mut nghttp2_session) -> usize ; }
};
}
