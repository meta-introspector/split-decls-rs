macro_rules! deps {
    () => {
        DecompressorOxide!();
        TINFLStatus!();
    };
}

macro_rules! decompress {
    () => {
        deps!();
        # [doc = " Main decompression function. Keeps decompressing data from `in_buf` until the `in_buf` is"] # [doc = " empty, `out` is full, the end of the deflate stream is hit, or there is an error in the"] # [doc = " deflate stream."] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " `r` is a [`DecompressorOxide`] struct with the state of this stream."] # [doc = ""] # [doc = " `in_buf` is a reference to the compressed data that is to be decompressed. The decompressor will"] # [doc = " start at the first byte of this buffer."] # [doc = ""] # [doc = " `out` is a reference to the buffer that will store the decompressed data, and that"] # [doc = " stores previously decompressed data if any."] # [doc = ""] # [doc = " * The offset given by `out_pos` indicates where in the output buffer slice writing should start."] # [doc = " * If [`TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF`] is not set, the output buffer is used in a"] # [doc = "   wrapping manner, and it's size is required to be a power of 2."] # [doc = " * The decompression function normally needs access to 32KiB of the previously decompressed data"] # [doc = "   (or to the beginning of the decompressed data if less than 32KiB has been decompressed.)"] # [doc = "     - If this data is not available, decompression may fail."] # [doc = "     - Some deflate compressors allow specifying a window size which limits match distances to"] # [doc = "       less than this, or alternatively an RLE mode where matches will only refer to the previous byte"] # [doc = "       and thus allows a smaller output buffer. The window size can be specified in the zlib"] # [doc = "       header structure, however, the header data should not be relied on to be correct."] # [doc = ""] # [doc = " `flags` indicates settings and status to the decompression function."] # [doc = " * The [`TINFL_FLAG_HAS_MORE_INPUT`] has to be specified if more compressed data is to be provided"] # [doc = "   in a subsequent call to this function."] # [doc = " * See the the [`inflate_flags`] module for details on other flags."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " Returns a tuple containing the status of the compressor, the number of input bytes read, and the"] # [doc = " number of bytes output to `out`."] pub fn decompress (r : & mut DecompressorOxide , in_buf : & [u8] , out : & mut [u8] , out_pos : usize , flags : u32 ,) -> (TINFLStatus , usize , usize) { decompress_with_limit (r , in_buf , out , out_pos , usize :: MAX , flags) }
    };
}

decompress!();