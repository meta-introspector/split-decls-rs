macro_rules! other_274 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Changes header table size of the |deflater| to"] # [doc = " |settings_max_dynamic_table_size| bytes.  This may trigger eviction"] # [doc = " in the dynamic table."] # [doc = ""] # [doc = " The |settings_max_dynamic_table_size| should be the value received"] # [doc = " in SETTINGS_HEADER_TABLE_SIZE."] # [doc = ""] # [doc = " The deflater never uses more memory than"] # [doc = " ``max_deflate_dynamic_table_size`` bytes specified in"] # [doc = " `nghttp2_hd_deflate_new()`.  Therefore, if"] # [doc = " |settings_max_dynamic_table_size| >"] # [doc = " ``max_deflate_dynamic_table_size``, resulting maximum table size"] # [doc = " becomes ``max_deflate_dynamic_table_size``."] # [doc = ""] # [doc = " This function returns 0 if it succeeds, or one of the following"] # [doc = " negative error codes:"] # [doc = ""] # [doc = " :enum:`NGHTTP2_ERR_NOMEM`"] # [doc = "     Out of memory."] pub fn nghttp2_hd_deflate_change_table_size (deflater : * mut nghttp2_hd_deflater , settings_max_dynamic_table_size : usize ,) -> :: std :: os :: raw :: c_int ; }
    };
}

other_274!();