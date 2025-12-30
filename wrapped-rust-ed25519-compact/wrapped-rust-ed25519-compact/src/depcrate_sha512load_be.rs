// Generated macro for load_be (function)
macro_rules! Depcrate_sha512load_be {
() => {
// Module: crate::sha512
// Provides: {"load_be"}
// Dependencies: {}
# [cfg_attr (feature = "opt_size" , inline (never))] # [cfg_attr (not (feature = "opt_size") , inline (always))] fn load_be (base : & [u8] , offset : usize) -> u64 { let addr = & base [offset ..] ; (addr [7] as u64) | (addr [6] as u64) << 8 | (addr [5] as u64) << 16 | (addr [4] as u64) << 24 | (addr [3] as u64) << 32 | (addr [2] as u64) << 40 | (addr [1] as u64) << 48 | (addr [0] as u64) << 56 }
};
}
