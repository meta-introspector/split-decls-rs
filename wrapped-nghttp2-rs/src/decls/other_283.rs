macro_rules! other_283 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Initializes |*inflater_ptr| for inflating name/values pairs."] # [doc = ""] # [doc = " If this function fails, |*inflater_ptr| is left untouched."] # [doc = ""] # [doc = " This function returns 0 if it succeeds, or one of the following"] # [doc = " negative error codes:"] # [doc = ""] # [doc = " :enum:`NGHTTP2_ERR_NOMEM`"] # [doc = "     Out of memory."] pub fn nghttp2_hd_inflate_new (inflater_ptr : * mut * mut nghttp2_hd_inflater ,) -> :: std :: os :: raw :: c_int ; }
    };
}

other_283!();