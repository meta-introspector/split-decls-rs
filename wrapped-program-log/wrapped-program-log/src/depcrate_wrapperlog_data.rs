// Generated macro for log_data (function)
macro_rules! Depcrate_wrapperlog_data {
() => {
// Module: crate::wrapper
// Provides: {"log_data"}
// Dependencies: {}
# [doc = " Print some slices as `base64`."] # [inline (always)] pub fn log_data (data : & [& [u8]]) { # [cfg (any (target_os = "solana" , target_arch = "bpf"))] unsafe { sol_log_data (data as * const _ as * const u8 , data . len () as u64) } ; # [cfg (not (any (target_os = "solana" , target_arch = "bpf")))] black_box (data) ; }
};
}
