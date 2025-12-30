// Generated macro for other_315 (other)
macro_rules! Depcrateother_315 {
() => {
// Module: crate
// Provides: {"other_315"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the first child stream of |stream| in dependency tree."] # [doc = " Returns NULL if there is no such stream."] pub fn nghttp2_stream_get_first_child (stream : * mut nghttp2_stream) -> * mut nghttp2_stream ; }
};
}
