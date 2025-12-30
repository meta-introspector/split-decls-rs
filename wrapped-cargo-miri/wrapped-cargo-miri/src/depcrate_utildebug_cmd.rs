// Generated macro for debug_cmd (function)
macro_rules! Depcrate_utildebug_cmd {
() => {
// Module: crate::util
// Provides: {"debug_cmd"}
// Dependencies: {}
# [doc = " Debug-print a command that is going to be run."] pub fn debug_cmd (prefix : & str , verbose : usize , cmd : & Command) { if verbose == 0 { return ; } eprintln ! ("{prefix} running command: {cmd:?}") ; }
};
}
