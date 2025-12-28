macro_rules! other_191 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " This option prevents the library from retaining closed streams to"] # [doc = " maintain the priority tree.  If this option is set to nonzero,"] # [doc = " applications can discard closed stream completely to save memory."] pub fn nghttp2_option_set_no_closed_streams (option : * mut nghttp2_option , val : :: std :: os :: raw :: c_int ,) ; }
    };
}

other_191!()