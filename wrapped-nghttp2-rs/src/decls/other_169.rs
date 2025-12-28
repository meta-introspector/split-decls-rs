macro_rules! other_169 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked when the library asks the"] # [doc = " application to unpack extension frame payload from wire format."] pub fn nghttp2_session_callbacks_set_unpack_extension_callback (cbs : * mut nghttp2_session_callbacks , unpack_extension_callback : nghttp2_unpack_extension_callback ,) ; }
    };
}

other_169!();