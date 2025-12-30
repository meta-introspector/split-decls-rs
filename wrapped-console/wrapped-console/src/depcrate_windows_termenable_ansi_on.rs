// Generated macro for enable_ansi_on (function)
macro_rules! Depcrate_windows_termenable_ansi_on {
() => {
// Module: crate::windows_term
// Provides: {"enable_ansi_on"}
// Dependencies: {}
fn enable_ansi_on (out : & Term) -> bool { set_console_mode (out . as_raw_handle () , ENABLE_VIRTUAL_TERMINAL_PROCESSING , true ,) . is_some () }
};
}
