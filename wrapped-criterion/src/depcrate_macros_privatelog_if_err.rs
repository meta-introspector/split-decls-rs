// Generated macro for log_if_err (macro)
macro_rules! Depcrate_macros_privatelog_if_err {
() => {
// Module: crate::macros_private
// Provides: {"log_if_err"}
// Dependencies: {}
# [doc = " Logs an error, ignores an `Ok` value."] macro_rules ! log_if_err { ($ x : expr) => { let closure = || { try_else_return ! ($ x) ; } ; closure () ; } ; }
};
}
