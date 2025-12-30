// Generated macro for store_be (function)
macro_rules! Depcrate_sha512store_be {
() => {
// Module: crate::sha512
// Provides: {"store_be"}
// Dependencies: {}
# [cfg_attr (feature = "opt_size" , inline (never))] # [cfg_attr (not (feature = "opt_size") , inline (always))] fn store_be (base : & mut [u8] , offset : usize , x : u64) { let addr = & mut base [offset ..] ; addr [7] = x as u8 ; addr [6] = (x >> 8) as u8 ; addr [5] = (x >> 16) as u8 ; addr [4] = (x >> 24) as u8 ; addr [3] = (x >> 32) as u8 ; addr [2] = (x >> 40) as u8 ; addr [1] = (x >> 48) as u8 ; addr [0] = (x >> 56) as u8 ; }
};
}
