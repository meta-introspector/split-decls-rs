// Generated macro for bail_span (macro)
macro_rules! Depcrate_errorbail_span {
() => {
// Module: crate::error
// Provides: {"bail_span"}
// Dependencies: {}
# [doc = " Immediately fail and return an Err, with the arguments passed to err_span!"] macro_rules ! bail_span { ($ ($ t : tt) *) => (return Err (err_span ! ($ ($ t) *) . into ())) }
};
}
