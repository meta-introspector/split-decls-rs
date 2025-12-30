// Generated macro for CompressedFileRange (struct)
macro_rules! Depcrate_readCompressedFileRange {
() => {
// Module: crate::read
// Provides: {"CompressedFileRange"}
// Dependencies: {}
# [doc = " A range in a file that may be compressed."] # [doc = ""] # [doc = " Returned by [`ObjectSection::compressed_file_range`]."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct CompressedFileRange { # [doc = " The data compression format."] pub format : CompressionFormat , # [doc = " The file offset of the compressed data."] pub offset : u64 , # [doc = " The compressed data size."] pub compressed_size : u64 , # [doc = " The uncompressed data size."] pub uncompressed_size : u64 , }
};
}
