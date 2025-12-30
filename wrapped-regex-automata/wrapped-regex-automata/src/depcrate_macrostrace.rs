// Generated macro for trace (macro)
macro_rules! Depcrate_macrostrace {
() => {
// Module: crate::macros
// Provides: {"trace"}
// Dependencies: {}
macro_rules ! trace { ($ ($ tt : tt) *) => { log ! (log :: trace ! ($ ($ tt) *)) } }
};
}
