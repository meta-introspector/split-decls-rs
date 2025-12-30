// Generated macro for compress_slice_with_flush (function)
macro_rules! Depcratecompress_slice_with_flush {
() => {
// Module: crate
// Provides: {"compress_slice_with_flush"}
// Dependencies: {}
# [allow (clippy :: too_many_arguments)] pub fn compress_slice_with_flush < 'a > (output : & 'a mut [u8] , input : & [u8] , level : i32 , method : i32 , window_bits : i32 , mem_level : i32 , strategy : i32 , final_flush : i32 ,) -> (& 'a mut [u8] , i32) { let output_uninit = unsafe { core :: slice :: from_raw_parts_mut (output . as_mut_ptr () as * mut MaybeUninit < u8 > , output . len ()) } ; compress_with_flush (output_uninit , input , level , method , window_bits , mem_level , strategy , final_flush ,) }
};
}
