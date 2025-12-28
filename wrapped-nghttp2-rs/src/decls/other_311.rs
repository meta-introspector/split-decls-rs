macro_rules! other_311 {
    () => {
        extern "C" { pub fn nghttp2_stream_get_stream_id (stream : * mut nghttp2_stream) -> i32 ; }
    };
}

other_311!()