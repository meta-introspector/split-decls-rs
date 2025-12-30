// Generated macro for trace (macro)
macro_rules! Depcrate_loggingtrace {
() => {
// Module: crate::logging
// Provides: {"trace"}
// Dependencies: {}
macro_rules ! trace { ($ ($ tt : tt) *) => { log ! (log :: trace ! ($ ($ tt) *)) } }
};
}
