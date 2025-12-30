// Generated macro for log_64 (function)
macro_rules! Depcrate_wrapperlog_64 {
() => {
// Module: crate::wrapper
// Provides: {"log_64"}
// Dependencies: {}
# [doc = " Print 64-bit values represented as hexadecimal to the log."] # [inline (always)] pub fn log_64 (arg1 : u64 , arg2 : u64 , arg3 : u64 , arg4 : u64 , arg5 : u64) { # [cfg (any (target_os = "solana" , target_arch = "bpf"))] unsafe { sol_log_64_ (arg1 , arg2 , arg3 , arg4 , arg5) ; } # [cfg (not (any (target_os = "solana" , target_arch = "bpf")))] black_box ((arg1 , arg2 , arg3 , arg4 , arg5)) ; }
};
}
