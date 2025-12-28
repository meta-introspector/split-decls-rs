macro_rules! other_187 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets extension frame type the application is willing to receive"] # [doc = " using builtin handler.  The |type| is the extension frame type to"] # [doc = " receive, and must be strictly greater than 0x9.  Otherwise, this"] # [doc = " function does nothing.  The application can call this function"] # [doc = " multiple times to set more than one frame type to receive.  The"] # [doc = " application does not have to call this function if it just sends"] # [doc = " extension frames."] # [doc = ""] # [doc = " If same frame type is passed to both"] # [doc = " `nghttp2_option_set_builtin_recv_extension_type()` and"] # [doc = " `nghttp2_option_set_user_recv_extension_type()`, the latter takes"] # [doc = " precedence."] pub fn nghttp2_option_set_builtin_recv_extension_type (option : * mut nghttp2_option , type_ : u8) ; }
    };
}

other_187!()