// Generated macro for compress_with_flush (function)
macro_rules! Depcratecompress_with_flush {
() => {
// Module: crate
// Provides: {"compress_with_flush"}
// Dependencies: {}
# [allow (clippy :: too_many_arguments)] pub fn compress_with_flush < 'a > (output : & 'a mut [MaybeUninit < u8 >] , input : & [u8] , level : i32 , method : i32 , window_bits : i32 , mem_level : i32 , strategy : i32 , final_flush : i32 ,) -> (& 'a mut [u8] , i32) { let mut stream = MaybeUninit :: zeroed () ; unsafe { let err = generated :: deflateInit2_ (stream . as_mut_ptr () , level , method , window_bits , mem_level , strategy , generated :: zlibVersion () , core :: mem :: size_of :: < generated :: z_stream > () as _ ,) ; if err != generated :: Z_OK { return (& mut [] , err) ; } } ; let stream = unsafe { stream . assume_init_mut () } ; stream . next_in = input . as_ptr () as * mut u8 ; stream . next_out = output . as_mut_ptr () as * mut u8 ; let max = core :: ffi :: c_uint :: MAX as usize ; let mut left = output . len () ; let mut source_len = input . len () ; loop { if stream . avail_out == 0 { stream . avail_out = Ord :: min (left , max) as _ ; left -= stream . avail_out as usize ; } if stream . avail_in == 0 { stream . avail_in = Ord :: min (source_len , max) as _ ; source_len -= stream . avail_in as usize ; } let flush = if source_len > 0 { generated :: Z_NO_FLUSH } else { final_flush } ; let err = unsafe { deflate (stream , flush) } ; if err != generated :: Z_OK { break ; } } let output_slice = unsafe { core :: slice :: from_raw_parts_mut (output . as_mut_ptr () as * mut u8 , stream . total_out as usize) } ; unsafe { let err = deflateEnd (stream) ; assert_eq ! (generated :: Z_OK , err) ; } (output_slice , generated :: Z_OK) }
};
}
