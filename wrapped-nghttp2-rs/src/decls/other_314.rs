macro_rules! other_314 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the first child stream of |stream| in dependency tree."] # [doc = " Returns NULL if there is no such stream."] pub fn nghttp2_stream_get_first_child (stream : * mut nghttp2_stream) -> * mut nghttp2_stream ; }
    };
}

other_314!();