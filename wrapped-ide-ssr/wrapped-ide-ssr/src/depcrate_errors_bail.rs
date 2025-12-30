// Generated macro for _bail (macro)
macro_rules! Depcrate_errors_bail {
() => {
// Module: crate::errors
// Provides: {"_bail"}
// Dependencies: {}
# [doc = " Returns from the current function with an error, supplied by arguments as for format!"] macro_rules ! _bail { ($ ($ tokens : tt) *) => { return Err (crate :: errors :: error ! ($ ($ tokens) *)) } }
};
}
