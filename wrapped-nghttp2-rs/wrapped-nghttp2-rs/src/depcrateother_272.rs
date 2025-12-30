// Generated macro for other_272 (other)
macro_rules! Depcrateother_272 {
() => {
// Module: crate
// Provides: {"other_272"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Initializes |*deflater_ptr| for deflating name/values pairs."] # [doc = ""] # [doc = " The |max_deflate_dynamic_table_size| is the upper bound of header"] # [doc = " table size the deflater will use."] # [doc = ""] # [doc = " If this function fails, |*deflater_ptr| is left untouched."] # [doc = ""] # [doc = " This function returns 0 if it succeeds, or one of the following"] # [doc = " negative error codes:"] # [doc = ""] # [doc = " :enum:`NGHTTP2_ERR_NOMEM`"] # [doc = "     Out of memory."] pub fn nghttp2_hd_deflate_new (deflater_ptr : * mut * mut nghttp2_hd_deflater , max_deflate_dynamic_table_size : usize ,) -> :: std :: os :: raw :: c_int ; }
};
}
