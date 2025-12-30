// Generated macro for other_279 (other)
macro_rules! Depcrateother_279 {
() => {
// Module: crate
// Provides: {"other_279"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the number of entries that header table of |deflater|"] # [doc = " contains.  This is the sum of the number of static table and"] # [doc = " dynamic table, so the return value is at least 61."] pub fn nghttp2_hd_deflate_get_num_table_entries (deflater : * mut nghttp2_hd_deflater) -> usize ; }
};
}
