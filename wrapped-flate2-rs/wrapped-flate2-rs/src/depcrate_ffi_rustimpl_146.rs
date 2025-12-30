// Generated macro for impl_146 (impl)
macro_rules! Depcrate_ffi_rustimpl_146 {
() => {
// Module: crate::ffi::rust
// Provides: {"impl_146"}
// Dependencies: {}
impl InflateBackend for Inflate { fn make (zlib_header : bool , _window_bits : u8) -> Self { let format = format_from_bool (zlib_header) ; Inflate { inner : InflateState :: new_boxed (format) , total_in : 0 , total_out : 0 , } } fn decompress (& mut self , input : & [u8] , output : & mut [u8] , flush : FlushDecompress ,) -> Result < Status , DecompressError > { let mz_flush = flush . into () ; let res = inflate :: stream :: inflate (& mut self . inner , input , output , mz_flush) ; self . total_in += res . bytes_consumed as u64 ; self . total_out += res . bytes_written as u64 ; match res . status { Ok (status) => match status { MZStatus :: Ok => Ok (Status :: Ok) , MZStatus :: StreamEnd => Ok (Status :: StreamEnd) , MZStatus :: NeedDict => { mem :: decompress_need_dict (self . inner . decompressor () . adler32 () . unwrap_or (0)) } } , Err (status) => match status { MZError :: Buf => Ok (Status :: BufError) , _ => mem :: decompress_failed (ErrorMessage) , } , } } fn reset (& mut self , zlib_header : bool) { self . inner . reset (format_from_bool (zlib_header)) ; self . total_in = 0 ; self . total_out = 0 ; } }
};
}
