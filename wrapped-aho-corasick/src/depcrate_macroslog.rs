// Generated macro for log (macro)
macro_rules! Depcrate_macroslog {
() => {
// Module: crate::macros
// Provides: {"log"}
// Dependencies: {}
macro_rules ! log { ($ ($ tt : tt) *) => { # [cfg (feature = "logging")] { $ ($ tt) * } } }
};
}
