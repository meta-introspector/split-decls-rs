// Generated macro for other_273 (other)
macro_rules! Depcrateother_273 {
() => {
// Module: crate
// Provides: {"other_273"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Like `nghttp2_hd_deflate_new()`, but with additional custom memory"] # [doc = " allocator specified in the |mem|."] # [doc = ""] # [doc = " The |mem| can be ``NULL`` and the call is equivalent to"] # [doc = " `nghttp2_hd_deflate_new()`."] # [doc = ""] # [doc = " This function does not take ownership |mem|.  The application is"] # [doc = " responsible for freeing |mem|."] # [doc = ""] # [doc = " The library code does not refer to |mem| pointer after this"] # [doc = " function returns, so the application can safely free it."] pub fn nghttp2_hd_deflate_new2 (deflater_ptr : * mut * mut nghttp2_hd_deflater , max_deflate_dynamic_table_size : usize , mem : * mut nghttp2_mem ,) -> :: std :: os :: raw :: c_int ; }
};
}
