// Generated macro for RejectOpWith (enum)
macro_rules! Depcrate_machineRejectOpWith {
() => {
// Module: crate::machine
// Provides: {"RejectOpWith"}
// Dependencies: {}
# [derive (Copy , Clone , Debug , PartialEq)] pub enum RejectOpWith { # [doc = " Isolated op is rejected with an abort of the machine."] Abort , # [doc = " If not Abort, miri returns an error for an isolated op."] # [doc = " Following options determine if user should be warned about such error."] # [doc = " Do not print warning about rejected isolated op."] NoWarning , # [doc = " Print a warning about rejected isolated op, with backtrace."] Warning , # [doc = " Print a warning about rejected isolated op, without backtrace."] WarningWithoutBacktrace , }
};
}
