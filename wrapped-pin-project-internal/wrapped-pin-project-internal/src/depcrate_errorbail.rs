// Generated macro for bail (macro)
macro_rules! Depcrate_errorbail {
() => {
// Module: crate::error
// Provides: {"bail"}
// Dependencies: {}
macro_rules ! bail { ($ ($ tt : tt) *) => { return Err (format_err ! ($ ($ tt) *)) } ; }
};
}
