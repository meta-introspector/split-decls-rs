// Generated macro for compress_slice (function)
macro_rules! Depcratecompress_slice {
() => {
// Module: crate
// Provides: {"compress_slice"}
// Dependencies: {}
pub fn compress_slice < 'a > (output : & 'a mut [u8] , input : & [u8] , level : i32 , method : i32 , window_bits : i32 , mem_level : i32 , strategy : i32 ,) -> (& 'a mut [u8] , i32) { compress_slice_with_flush (output , input , level , method , window_bits , mem_level , strategy , generated :: Z_FINISH ,) }
};
}
