// Generated macro for impl_122 (impl)
macro_rules! Depcrate_ffi_cimpl_122 {
() => {
// Module: crate::ffi::c
// Provides: {"impl_122"}
// Dependencies: {}
impl DeflateBackend for Deflate { fn make (level : Compression , zlib_header : bool , window_bits : u8) -> Self { unsafe { let state = StreamWrapper :: default () ; let ret = mz_deflateInit2 (state . inner , level . 0 as c_int , MZ_DEFLATED , if zlib_header { window_bits as c_int } else { - (window_bits as c_int) } , 8 , MZ_DEFAULT_STRATEGY ,) ; assert_eq ! (ret , 0) ; Deflate { inner : Stream { stream_wrapper : state , total_in : 0 , total_out : 0 , _marker : marker :: PhantomData , } , } } } fn compress (& mut self , input : & [u8] , output : & mut [u8] , flush : FlushCompress ,) -> Result < Status , CompressError > { unsafe { self . compress_inner (input , output . as_mut_ptr () , output . len () , flush) } } fn compress_uninit (& mut self , input : & [u8] , output : & mut [MaybeUninit < u8 >] , flush : FlushCompress ,) -> Result < Status , CompressError > { unsafe { self . compress_inner (input , output . as_mut_ptr () as * mut _ , output . len () , flush) } } fn reset (& mut self) { self . inner . total_in = 0 ; self . inner . total_out = 0 ; let rc = unsafe { mz_deflateReset (self . inner . stream_wrapper . inner) } ; assert_eq ! (rc , MZ_OK) ; } }
};
}
