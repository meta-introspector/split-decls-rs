macro_rules! other_45 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Increments the reference count of |rcbuf| by 1."] pub fn nghttp2_rcbuf_incref (rcbuf : * mut nghttp2_rcbuf) ; }
    };
}

other_45!();