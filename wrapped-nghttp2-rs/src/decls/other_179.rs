macro_rules! other_179 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Initializes |*option_ptr| with default values."] # [doc = ""] # [doc = " When the application finished using this object, it can use"] # [doc = " `nghttp2_option_del()` to free its memory."] # [doc = ""] # [doc = " This function returns 0 if it succeeds, or one of the following"] # [doc = " negative error codes:"] # [doc = ""] # [doc = " :enum:`NGHTTP2_ERR_NOMEM`"] # [doc = "     Out of memory."] pub fn nghttp2_option_new (option_ptr : * mut * mut nghttp2_option) -> :: std :: os :: raw :: c_int ; }
    };
}

other_179!()