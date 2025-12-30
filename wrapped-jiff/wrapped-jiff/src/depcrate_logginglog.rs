// Generated macro for log (macro)
macro_rules! Depcrate_logginglog {
() => {
// Module: crate::logging
// Provides: {"log"}
// Dependencies: {}
macro_rules ! log { ($ ($ tt : tt) *) => { # [cfg (feature = "logging")] { $ ($ tt) * } } }
};
}
