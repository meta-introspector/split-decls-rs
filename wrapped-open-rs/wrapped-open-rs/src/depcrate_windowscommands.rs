// Generated macro for commands (function)
macro_rules! Depcrate_windowscommands {
() => {
// Module: crate::windows
// Provides: {"commands"}
// Dependencies: {}
pub fn commands < T : AsRef < OsStr > > (path : T) -> Vec < Command > { let mut cmd = Command :: new ("cmd") ; cmd . arg ("/c") . arg ("start") . raw_arg ("\"\"") . raw_arg (wrap_in_quotes (path)) . creation_flags (CREATE_NO_WINDOW) ; vec ! [cmd] }
};
}
