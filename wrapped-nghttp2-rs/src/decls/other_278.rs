macro_rules! other_278 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the number of entries that header table of |deflater|"] # [doc = " contains.  This is the sum of the number of static table and"] # [doc = " dynamic table, so the return value is at least 61."] pub fn nghttp2_hd_deflate_get_num_table_entries (deflater : * mut nghttp2_hd_deflater) -> usize ; }
    };
}

other_278!()