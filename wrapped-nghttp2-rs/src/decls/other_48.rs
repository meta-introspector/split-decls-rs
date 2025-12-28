macro_rules! other_48 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns nonzero if the underlying buffer is statically allocated,"] # [doc = " and 0 otherwise. This can be useful for language bindings that wish"] # [doc = " to avoid creating duplicate strings for these buffers."] pub fn nghttp2_rcbuf_is_static (rcbuf : * const nghttp2_rcbuf) -> :: std :: os :: raw :: c_int ; }
    };
}

other_48!()