macro_rules! other_220 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the current dynamic table size of HPACK inflater, including"] # [doc = " the overhead 32 bytes per entry described in RFC 7541."] pub fn nghttp2_session_get_hd_inflate_dynamic_table_size (session : * mut nghttp2_session ,) -> usize ; }
    };
}

other_220!()