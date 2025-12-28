macro_rules! other_293 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Signals the end of decompression for one header block."] # [doc = ""] # [doc = " This function returns 0 if it succeeds. Currently this function"] # [doc = " always succeeds."] pub fn nghttp2_hd_inflate_end_headers (inflater : * mut nghttp2_hd_inflater ,) -> :: std :: os :: raw :: c_int ; }
    };
}

other_293!();