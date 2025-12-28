macro_rules! other_168 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked when the library asks the"] # [doc = " application to pack extension frame payload in wire format."] pub fn nghttp2_session_callbacks_set_pack_extension_callback (cbs : * mut nghttp2_session_callbacks , pack_extension_callback : nghttp2_pack_extension_callback ,) ; }
    };
}

other_168!()