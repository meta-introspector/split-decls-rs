// Generated macro for err (macro)
macro_rules! Depcrate_errorerr {
() => {
// Module: crate::error
// Provides: {"err"}
// Dependencies: {}
# [doc = " Creates a new ad hoc error via `format_args!`."] macro_rules ! err { ($ ($ tt : tt) *) => { { crate :: error :: Error :: adhoc_from_args (format_args ! ($ ($ tt) *)) } } }
};
}
