// Generated macro for InflateBackend (trait)
macro_rules! Depcrate_ffiInflateBackend {
() => {
// Module: crate::ffi
// Provides: {"InflateBackend"}
// Dependencies: {}
pub trait InflateBackend : Backend { fn make (zlib_header : bool , window_bits : u8) -> Self ; fn decompress (& mut self , input : & [u8] , output : & mut [u8] , flush : FlushDecompress ,) -> Result < Status , DecompressError > ; fn decompress_uninit (& mut self , input : & [u8] , output : & mut [MaybeUninit < u8 >] , flush : FlushDecompress ,) -> Result < Status , DecompressError > { self . decompress (input , initialize_buffer (output) , flush) } fn reset (& mut self , zlib_header : bool) ; }
};
}
