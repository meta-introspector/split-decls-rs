macro_rules! other_46 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Decrements the reference count of |rcbuf| by 1.  If the reference"] # [doc = " count becomes zero, the object pointed by |rcbuf| will be freed."] # [doc = " In this case, application must not use |rcbuf| again."] pub fn nghttp2_rcbuf_decref (rcbuf : * mut nghttp2_rcbuf) ; }
    };
}

other_46!();