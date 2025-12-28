macro_rules! other_170 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked when chunk of extension frame"] # [doc = " payload is received."] pub fn nghttp2_session_callbacks_set_on_extension_chunk_recv_callback (cbs : * mut nghttp2_session_callbacks , on_extension_chunk_recv_callback : nghttp2_on_extension_chunk_recv_callback ,) ; }
    };
}

other_170!()