macro_rules! other_277 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns an upper bound on the compressed size after deflation of"] # [doc = " |nva| of length |nvlen|."] pub fn nghttp2_hd_deflate_bound (deflater : * mut nghttp2_hd_deflater , nva : * const nghttp2_nv , nvlen : usize ,) -> usize ; }
    };
}

other_277!();