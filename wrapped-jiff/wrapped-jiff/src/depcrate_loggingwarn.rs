// Generated macro for warn (macro)
macro_rules! Depcrate_loggingwarn {
() => {
// Module: crate::logging
// Provides: {"warn"}
// Dependencies: {}
macro_rules ! warn { ($ ($ tt : tt) *) => { log ! (log :: warn ! ($ ($ tt) *)) } }
};
}
