// Generated macro for no_color (function)
macro_rules! Depcrate_loggingno_color {
() => {
// Module: crate::logging
// Provides: {"no_color"}
// Dependencies: {}
fn no_color () -> bool { option_env ! ("NO_COLOR") . is_some_and (| val | ! val . is_empty ()) }
};
}
