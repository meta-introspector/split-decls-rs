macro_rules! other_164 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked when the library asks application"] # [doc = " how many padding bytes are required for the transmission of the"] # [doc = " given frame."] pub fn nghttp2_session_callbacks_set_select_padding_callback (cbs : * mut nghttp2_session_callbacks , select_padding_callback : nghttp2_select_padding_callback ,) ; }
    };
}

other_164!();