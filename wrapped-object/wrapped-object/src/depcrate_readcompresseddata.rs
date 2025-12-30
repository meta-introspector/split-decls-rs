// Generated macro for CompressedData (struct)
macro_rules! Depcrate_readCompressedData {
() => {
// Module: crate::read
// Provides: {"CompressedData"}
// Dependencies: {}
# [doc = " Data that may be compressed."] # [doc = ""] # [doc = " Returned by [`ObjectSection::compressed_data`]."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct CompressedData < 'data > { # [doc = " The data compression format."] pub format : CompressionFormat , # [doc = " The compressed data."] pub data : & 'data [u8] , # [doc = " The uncompressed data size."] pub uncompressed_size : u64 , }
};
}
