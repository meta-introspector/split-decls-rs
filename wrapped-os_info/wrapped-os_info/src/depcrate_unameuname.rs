// Generated macro for uname (function)
macro_rules! Depcrate_unameuname {
() => {
// Module: crate::uname
// Provides: {"uname"}
// Dependencies: {}
pub fn uname (field : UnameField) -> Option < String > { field . get_from_syscall () . or_else (| | uname_cli (field . cli_arg_name ())) }
};
}
