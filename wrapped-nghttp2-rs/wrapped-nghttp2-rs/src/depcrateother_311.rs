// Generated macro for other_311 (other)
macro_rules! Depcrateother_311 {
() => {
// Module: crate
// Provides: {"other_311"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the parent stream of |stream| in dependency tree.  Returns"] # [doc = " NULL if there is no such stream."] pub fn nghttp2_stream_get_parent (stream : * mut nghttp2_stream) -> * mut nghttp2_stream ; }
};
}
