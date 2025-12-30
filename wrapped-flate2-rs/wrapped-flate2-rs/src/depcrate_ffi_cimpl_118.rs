// Generated macro for impl_118 (impl)
macro_rules! Depcrate_ffi_cimpl_118 {
() => {
// Module: crate::ffi::c
// Provides: {"impl_118"}
// Dependencies: {}
impl InflateBackend for Inflate { fn make (zlib_header : bool , window_bits : u8) -> Self { unsafe { let state = StreamWrapper :: default () ; let ret = mz_inflateInit2 (state . inner , if zlib_header { window_bits as c_int } else { - (window_bits as c_int) } ,) ; assert_eq ! (ret , 0) ; Inflate { inner : Stream { stream_wrapper : state , total_in : 0 , total_out : 0 , _marker : marker :: PhantomData , } , } } } fn decompress (& mut self , input : & [u8] , output : & mut [u8] , flush : FlushDecompress ,) -> Result < Status , DecompressError > { unsafe { self . decompress_inner (input , output . as_mut_ptr () , output . len () , flush) } } fn decompress_uninit (& mut self , input : & [u8] , output : & mut [MaybeUninit < u8 >] , flush : FlushDecompress ,) -> Result < Status , DecompressError > { unsafe { self . decompress_inner (input , output . as_mut_ptr () as * mut _ , output . len () , flush) } } fn reset (& mut self , zlib_header : bool) { let bits = if zlib_header { MZ_DEFAULT_WINDOW_BITS } else { - MZ_DEFAULT_WINDOW_BITS } ; unsafe { inflateReset2 (self . inner . stream_wrapper . inner , bits) ; } self . inner . total_out = 0 ; self . inner . total_in = 0 ; } }
};
}
