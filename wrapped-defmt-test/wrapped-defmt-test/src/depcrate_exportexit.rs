// Generated macro for exit (function)
macro_rules! Depcrate_exportexit {
() => {
// Module: crate::export
// Provides: {"exit"}
// Dependencies: {}
# [doc = " Terminates the application and makes a semihosting-capable debug tool exit"] # [doc = " with status code 0."] pub fn exit () -> ! { loop { debug :: exit (debug :: EXIT_SUCCESS) ; } }
};
}
