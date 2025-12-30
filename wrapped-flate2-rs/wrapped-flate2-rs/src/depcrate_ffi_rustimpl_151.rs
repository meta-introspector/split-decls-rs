// Generated macro for impl_151 (impl)
macro_rules! Depcrate_ffi_rustimpl_151 {
() => {
// Module: crate::ffi::rust
// Provides: {"impl_151"}
// Dependencies: {}
impl DeflateBackend for Deflate { fn make (level : Compression , zlib_header : bool , _window_bits : u8) -> Self { debug_assert ! (level . level () <= 10) ; let mut inner : Box < CompressorOxide > = Box :: default () ; let format = format_from_bool (zlib_header) ; inner . set_format_and_level (format , level . level () . try_into () . unwrap_or (1)) ; Deflate { inner , total_in : 0 , total_out : 0 , } } fn compress (& mut self , input : & [u8] , output : & mut [u8] , flush : FlushCompress ,) -> Result < Status , CompressError > { let mz_flush = flush . into () ; let res = deflate :: stream :: deflate (& mut self . inner , input , output , mz_flush) ; self . total_in += res . bytes_consumed as u64 ; self . total_out += res . bytes_written as u64 ; match res . status { Ok (status) => match status { MZStatus :: Ok => Ok (Status :: Ok) , MZStatus :: StreamEnd => Ok (Status :: StreamEnd) , MZStatus :: NeedDict => mem :: compress_failed (ErrorMessage) , } , Err (status) => match status { MZError :: Buf => Ok (Status :: BufError) , _ => mem :: compress_failed (ErrorMessage) , } , } } fn reset (& mut self) { self . total_in = 0 ; self . total_out = 0 ; self . inner . reset () ; } }
};
}
