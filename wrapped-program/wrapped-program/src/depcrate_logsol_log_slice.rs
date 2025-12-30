// Generated macro for sol_log_slice (function)
macro_rules! Depcrate_logsol_log_slice {
() => {
// Module: crate::log
// Provides: {"sol_log_slice"}
// Dependencies: {}
# [doc = " Print the hexadecimal representation of a slice."] pub fn sol_log_slice (slice : & [u8]) { for (i , s) in slice . iter () . enumerate () { sol_log_64 (0 , 0 , 0 , i as u64 , * s as u64) ; } }
};
}
