// Generated macro for le64 (function)
macro_rules! Depcrate_paele64 {
() => {
// Module: crate::pae
// Provides: {"le64"}
// Dependencies: {}
# [doc = " Encode `n` to little-endian bytes. The MSB is cleared."] pub fn le64 (n : u64) -> [u8 ; size_of :: < u64 > ()] { let mut out = [0u8 ; size_of :: < u64 > ()] ; let mut n_tmp = n ; out [0] = (n_tmp & 255) as u8 ; n_tmp >>= 8 ; out [1] = (n_tmp & 255) as u8 ; n_tmp >>= 8 ; out [2] = (n_tmp & 255) as u8 ; n_tmp >>= 8 ; out [3] = (n_tmp & 255) as u8 ; n_tmp >>= 8 ; out [4] = (n_tmp & 255) as u8 ; n_tmp >>= 8 ; out [5] = (n_tmp & 255) as u8 ; n_tmp >>= 8 ; out [6] = (n_tmp & 255) as u8 ; n_tmp >>= 8 ; n_tmp &= 127 ; out [7] = (n_tmp & 255) as u8 ; out }
};
}
