// Generated macro for wrap_in_quotes (function)
macro_rules! Depcrate_windowswrap_in_quotes {
() => {
// Module: crate::windows
// Provides: {"wrap_in_quotes"}
// Dependencies: {}
fn wrap_in_quotes < T : AsRef < OsStr > > (path : T) -> OsString { let mut result = OsString :: from ("\"") ; result . push (path) ; result . push ("\"") ; result }
};
}
