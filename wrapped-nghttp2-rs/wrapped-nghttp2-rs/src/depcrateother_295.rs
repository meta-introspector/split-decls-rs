// Generated macro for other_295 (other)
macro_rules! Depcrateother_295 {
() => {
// Module: crate
// Provides: {"other_295"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the number of entries that header table of |inflater|"] # [doc = " contains.  This is the sum of the number of static table and"] # [doc = " dynamic table, so the return value is at least 61."] pub fn nghttp2_hd_inflate_get_num_table_entries (inflater : * mut nghttp2_hd_inflater) -> usize ; }
};
}
