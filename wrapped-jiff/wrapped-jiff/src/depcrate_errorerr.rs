// Generated macro for err (macro)
macro_rules! Depcrate_errorerr {
() => {
// Module: crate::error
// Provides: {"err"}
// Dependencies: {}
# [doc = " Creates a new ad hoc error with no causal chain."] # [doc = ""] # [doc = " This accepts the same arguments as the `format!` macro. The error it"] # [doc = " creates is just a wrapper around the string created by `format!`."] macro_rules ! err { ($ ($ tt : tt) *) => { { crate :: error :: Error :: adhoc_from_args (format_args ! ($ ($ tt) *)) } } }
};
}
