// Generated macro for impl_1322 (impl)
macro_rules! Depcrate_readimpl_1322 {
() => {
// Module: crate::read
// Provides: {"impl_1322"}
// Dependencies: {}
impl CompressedFileRange { # [doc = " Data that is uncompressed."] # [inline] pub fn none (range : Option < (u64 , u64) >) -> Self { if let Some ((offset , size)) = range { CompressedFileRange { format : CompressionFormat :: None , offset , compressed_size : size , uncompressed_size : size , } } else { CompressedFileRange { format : CompressionFormat :: None , offset : 0 , compressed_size : 0 , uncompressed_size : 0 , } } } # [doc = " Convert to [`CompressedData`] by reading from the file."] pub fn data < 'data , R : ReadRef < 'data > > (self , file : R) -> Result < CompressedData < 'data > > { let data = file . read_bytes_at (self . offset , self . compressed_size) . read_error ("Invalid compressed data size or offset") ? ; Ok (CompressedData { format : self . format , data , uncompressed_size : self . uncompressed_size , }) } }
};
}
