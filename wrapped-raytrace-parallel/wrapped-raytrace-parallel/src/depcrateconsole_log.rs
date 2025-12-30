// Generated macro for console_log (macro)
macro_rules! Depcrateconsole_log {
() => {
// Module: crate
// Provides: {"console_log"}
// Dependencies: {}
macro_rules ! console_log { ($ ($ t : tt) *) => (crate :: log (& format_args ! ($ ($ t) *) . to_string ())) }
};
}
