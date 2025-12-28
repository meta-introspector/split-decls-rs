macro_rules! other_279 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the table entry denoted by |idx| from header table of"] # [doc = " |deflater|.  The |idx| is 1-based, and idx=1 returns first entry of"] # [doc = " static table.  idx=62 returns first entry of dynamic table if it"] # [doc = " exists.  Specifying idx=0 is error, and this function returns NULL."] # [doc = " If |idx| is strictly greater than the number of entries the tables"] # [doc = " contain, this function returns NULL."] pub fn nghttp2_hd_deflate_get_table_entry (deflater : * mut nghttp2_hd_deflater , idx : usize ,) -> * const nghttp2_nv ; }
    };
}

other_279!();