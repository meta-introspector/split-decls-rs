// Generated macro for sol_log_64 (function)
macro_rules! Depcrate_logsol_log_64 {
() => {
// Module: crate::log
// Provides: {"sol_log_64"}
// Dependencies: {}
# [doc = " Print 64-bit values represented as hexadecimal to the log."] # [inline] pub fn sol_log_64 (arg1 : u64 , arg2 : u64 , arg3 : u64 , arg4 : u64 , arg5 : u64) { # [cfg (target_os = "solana")] unsafe { crate :: syscalls :: sol_log_64_ (arg1 , arg2 , arg3 , arg4 , arg5) ; } # [cfg (not (target_os = "solana"))] crate :: program_stubs :: sol_log_64 (arg1 , arg2 , arg3 , arg4 , arg5) ; }
};
}
