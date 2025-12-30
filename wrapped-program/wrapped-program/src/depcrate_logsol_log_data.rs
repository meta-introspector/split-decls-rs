// Generated macro for sol_log_data (function)
macro_rules! Depcrate_logsol_log_data {
() => {
// Module: crate::log
// Provides: {"sol_log_data"}
// Dependencies: {}
# [doc = " Print some slices as base64."] pub fn sol_log_data (data : & [& [u8]]) { # [cfg (target_os = "solana")] unsafe { crate :: syscalls :: sol_log_data (data as * const _ as * const u8 , data . len () as u64) } ; # [cfg (not (target_os = "solana"))] crate :: program_stubs :: sol_log_data (data) ; }
};
}
