macro_rules! other_186 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets extension frame type the application is willing to handle with"] # [doc = " user defined callbacks (see"] # [doc = " :type:`nghttp2_on_extension_chunk_recv_callback` and"] # [doc = " :type:`nghttp2_unpack_extension_callback`).  The |type| is"] # [doc = " extension frame type, and must be strictly greater than 0x9."] # [doc = " Otherwise, this function does nothing.  The application can call"] # [doc = " this function multiple times to set more than one frame type to"] # [doc = " receive.  The application does not have to call this function if it"] # [doc = " just sends extension frames."] pub fn nghttp2_option_set_user_recv_extension_type (option : * mut nghttp2_option , type_ : u8) ; }
    };
}

other_186!();