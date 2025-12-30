// Generated macro for decompress_to_vec_zlib_with_limit (function)
macro_rules! Depcrate_inflatedecompress_to_vec_zlib_with_limit {
() => {
// Module: crate::inflate
// Provides: {"decompress_to_vec_zlib_with_limit"}
// Dependencies: {}
# [doc = " Decompress the deflate-encoded data (with a zlib wrapper) in `input` to a vector."] # [doc = " The vector is grown to at most `max_size` bytes; if the data does not fit in that size,"] # [doc = " the error [struct][DecompressError] will contain the status [`TINFLStatus::HasMoreOutput`] and the data that was decompressed on failure."] # [doc = ""] # [doc = " As this function tries to decompress everything in one go, it's not ideal for general use outside of tests or where the output size is expected to be small."] # [doc = " It is suggested to use streaming decompression via the [flate2](https://github.com/alexcrichton/flate2-rs) library instead."] # [doc = ""] # [doc = " Returns a [`Result`] containing the [`Vec`] of decompressed data on success, and a [struct][DecompressError] on failure."] # [inline] # [cfg (feature = "with-alloc")] pub fn decompress_to_vec_zlib_with_limit (input : & [u8] , max_size : usize ,) -> Result < Vec < u8 > , DecompressError > { decompress_to_vec_inner (input , inflate_flags :: TINFL_FLAG_PARSE_ZLIB_HEADER , max_size) }
};
}
