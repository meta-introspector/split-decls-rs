// Generated macro for with_command (function)
macro_rules! Depcrate_windowswith_command {
() => {
// Module: crate::windows
// Provides: {"with_command"}
// Dependencies: {}
pub fn with_command < T : AsRef < OsStr > > (path : T , app : impl Into < String >) -> Command { let mut cmd = Command :: new ("cmd") ; cmd . arg ("/c") . arg ("start") . raw_arg ("\"\"") . raw_arg (wrap_in_quotes (app . into ())) . raw_arg (wrap_in_quotes (path)) . creation_flags (CREATE_NO_WINDOW) ; cmd }
};
}
