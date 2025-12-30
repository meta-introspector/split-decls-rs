// Generated macro for sol_log (function)
macro_rules! Depcratesol_log {
() => {
// Module: crate
// Provides: {"sol_log"}
// Dependencies: {}
# [doc = " Print a string to the log."] # [inline] pub fn sol_log (message : & str) { # [cfg (target_os = "solana")] unsafe { syscalls :: sol_log_ (message . as_ptr () , message . len () as u64) ; } # [cfg (all (not (target_os = "solana") , feature = "std"))] std :: println ! ("{message}") ; # [cfg (all (not (target_os = "solana") , not (feature = "std")))] core :: hint :: black_box (message) ; }
};
}
