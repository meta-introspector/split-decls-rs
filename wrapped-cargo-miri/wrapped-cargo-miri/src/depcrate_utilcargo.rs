// Generated macro for cargo (function)
macro_rules! Depcrate_utilcargo {
() => {
// Module: crate::util
// Provides: {"cargo"}
// Dependencies: {}
pub fn cargo () -> Command { Command :: new (env :: var_os ("CARGO") . unwrap_or_else (| | OsString :: from ("cargo"))) }
};
}
