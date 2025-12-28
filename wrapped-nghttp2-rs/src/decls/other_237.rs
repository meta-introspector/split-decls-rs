macro_rules! other_237 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns string describing the |lib_error_code|.  The"] # [doc = " |lib_error_code| must be one of the :enum:`nghttp2_error`."] pub fn nghttp2_strerror (lib_error_code : :: std :: os :: raw :: c_int) -> * const :: std :: os :: raw :: c_char ; }
    };
}

other_237!()