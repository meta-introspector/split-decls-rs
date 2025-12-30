// Generated macro for err (macro)
macro_rules! Depcrate_shared_util_errorerr {
() => {
// Module: crate::shared::util::error
// Provides: {"err"}
// Dependencies: {}
macro_rules ! err { ($ ($ tt : tt) *) => { { crate :: shared :: util :: error :: Error :: from_args (format_args ! ($ ($ tt) *)) } } }
};
}
