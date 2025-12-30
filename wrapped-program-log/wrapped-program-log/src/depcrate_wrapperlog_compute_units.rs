// Generated macro for log_compute_units (function)
macro_rules! Depcrate_wrapperlog_compute_units {
() => {
// Module: crate::wrapper
// Provides: {"log_compute_units"}
// Dependencies: {}
# [doc = " Print the remaining compute units available to the program."] # [inline] pub fn log_compute_units () { # [cfg (any (target_os = "solana" , target_arch = "bpf"))] unsafe { sol_log_compute_units_ () ; } }
};
}
