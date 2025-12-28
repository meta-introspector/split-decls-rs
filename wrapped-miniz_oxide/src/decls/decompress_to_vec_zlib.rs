macro_rules! deps {
    () => {
        Result!();
        DecompressError!();
    };
}

macro_rules! decompress_to_vec_zlib {
    () => {
        deps!();
        # [doc = " Decompress the deflate-encoded data (with a zlib wrapper) in `input` to a vector."] # [doc = ""] # [doc = " NOTE: This function will not bound the output, so if the output is large enough it can result in an out of memory error."] # [doc = " It is therefore suggested to not use this for anything other than test programs, use the functions with a specified limit, or"] # [doc = " ideally streaming decompression via the [flate2](https://github.com/alexcrichton/flate2-rs) library instead."] # [doc = ""] # [doc = " Returns a [`Result`] containing the [`Vec`] of decompressed data on success, and a [struct][DecompressError] containing the status and so far decompressed data if any on failure."] # [inline] # [cfg (feature = "with-alloc")] pub fn decompress_to_vec_zlib (input : & [u8]) -> Result < Vec < u8 > , DecompressError > { decompress_to_vec_inner (input , inflate_flags :: TINFL_FLAG_PARSE_ZLIB_HEADER , usize :: MAX ,) }
    };
}

decompress_to_vec_zlib!()