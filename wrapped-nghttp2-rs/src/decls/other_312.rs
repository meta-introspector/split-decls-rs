macro_rules! other_312 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the next sibling stream of |stream| in dependency tree."] # [doc = " Returns NULL if there is no such stream."] pub fn nghttp2_stream_get_next_sibling (stream : * mut nghttp2_stream) -> * mut nghttp2_stream ; }
    };
}

other_312!();