// Generated macro for impl_48 (impl)
macro_rules! Depcrate_llvm_utilsimpl_48 {
() => {
// Module: crate::llvm_utils
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'a > Parser < 'a > { # [doc = " Reads a sequence of:"] # [doc = " - Length of uncompressed data in bytes, as ULEB128"] # [doc = " - Length of compressed data in bytes (or 0), as ULEB128"] # [doc = " - The indicated number of compressed or uncompressed bytes"] # [doc = ""] # [doc = " If the number of compressed bytes is 0, the subsequent bytes are"] # [doc = " uncompressed. Otherwise, the subsequent bytes are compressed, and will"] # [doc = " be decompressed."] # [doc = ""] # [doc = " Returns the uncompressed bytes that were read directly or decompressed."] pub (crate) fn read_chunk_to_uncompressed_bytes (& mut self) -> anyhow :: Result < Cow < 'a , [u8] > > { let uncompressed_len = self . read_uleb128_usize () ? ; let compressed_len = self . read_uleb128_usize () ? ; if compressed_len == 0 { let uncompressed_bytes = self . read_n_bytes (uncompressed_len) ? ; Ok (Cow :: Borrowed (uncompressed_bytes)) } else { let compressed_bytes = self . read_n_bytes (compressed_len) ? ; let uncompressed_bytes = miniz_oxide :: inflate :: decompress_to_vec_zlib_with_limit (compressed_bytes , uncompressed_len ,) . map_err (| e | anyhow ! ("{e:?}")) ? ; ensure ! (uncompressed_bytes . len () == uncompressed_len) ; Ok (Cow :: Owned (uncompressed_bytes)) } } }
};
}
