// Generated macro for sol_log_compute_units (function)
macro_rules! Depcrate_logsol_log_compute_units {
() => {
// Module: crate::log
// Provides: {"sol_log_compute_units"}
// Dependencies: {}
# [doc = " Print the remaining compute units available to the program."] # [inline] pub fn sol_log_compute_units () { # [cfg (target_os = "solana")] unsafe { crate :: syscalls :: sol_log_compute_units_ () ; } # [cfg (not (target_os = "solana"))] crate :: program_stubs :: sol_log_compute_units () ; }
};
}
