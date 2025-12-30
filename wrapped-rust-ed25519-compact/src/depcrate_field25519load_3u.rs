// Generated macro for load_3u (function)
macro_rules! Depcrate_field25519load_3u {
() => {
// Module: crate::field25519
// Provides: {"load_3u"}
// Dependencies: {}
# [cfg_attr (feature = "opt_size" , inline (never))] # [cfg_attr (not (feature = "opt_size") , inline)] pub fn load_3u (s : & [u8]) -> u64 { (s [0] as u64) | ((s [1] as u64) << 8) | ((s [2] as u64) << 16) }
};
}
