// Generated macro for other_278 (other)
macro_rules! Depcrateother_278 {
() => {
// Module: crate
// Provides: {"other_278"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns an upper bound on the compressed size after deflation of"] # [doc = " |nva| of length |nvlen|."] pub fn nghttp2_hd_deflate_bound (deflater : * mut nghttp2_hd_deflater , nva : * const nghttp2_nv , nvlen : usize ,) -> usize ; }
};
}
