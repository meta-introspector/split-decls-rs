// Generated macro for DeflateBackend (trait)
macro_rules! Depcrate_ffiDeflateBackend {
() => {
// Module: crate::ffi
// Provides: {"DeflateBackend"}
// Dependencies: {}
pub trait DeflateBackend : Backend { fn make (level : Compression , zlib_header : bool , window_bits : u8) -> Self ; fn compress (& mut self , input : & [u8] , output : & mut [u8] , flush : FlushCompress ,) -> Result < Status , CompressError > ; fn compress_uninit (& mut self , input : & [u8] , output : & mut [MaybeUninit < u8 >] , flush : FlushCompress ,) -> Result < Status , CompressError > { self . compress (input , initialize_buffer (output) , flush) } fn reset (& mut self) ; }
};
}
