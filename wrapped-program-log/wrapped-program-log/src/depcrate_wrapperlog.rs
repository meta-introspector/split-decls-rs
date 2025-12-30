// Generated macro for log (function)
macro_rules! Depcrate_wrapperlog {
() => {
// Module: crate::wrapper
// Provides: {"log"}
// Dependencies: {}
# [doc = " Print a string to the log."] # [inline (always)] pub fn log (message : & str) { # [cfg (any (target_os = "solana" , target_arch = "bpf"))] unsafe { sol_log_ (message . as_ptr () , message . len () as u64) ; } # [cfg (not (any (target_os = "solana" , target_arch = "bpf")))] black_box (message) ; }
};
}
