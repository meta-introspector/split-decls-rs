macro_rules! other_310 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the parent stream of |stream| in dependency tree.  Returns"] # [doc = " NULL if there is no such stream."] pub fn nghttp2_stream_get_parent (stream : * mut nghttp2_stream) -> * mut nghttp2_stream ; }
    };
}

other_310!();