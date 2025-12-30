// Generated macro for other_313 (other)
macro_rules! Depcrateother_313 {
() => {
// Module: crate
// Provides: {"other_313"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the next sibling stream of |stream| in dependency tree."] # [doc = " Returns NULL if there is no such stream."] pub fn nghttp2_stream_get_next_sibling (stream : * mut nghttp2_stream) -> * mut nghttp2_stream ; }
};
}
