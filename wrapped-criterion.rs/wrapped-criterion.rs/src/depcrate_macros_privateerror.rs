// Generated macro for error (macro)
macro_rules! Depcrate_macros_privateerror {
() => {
// Module: crate::macros_private
// Provides: {"error"}
// Dependencies: {}
# [doc = " Print an error message to stdout. Format is the same as println! or format!"] macro_rules ! error { ($ ($ arg : tt) *) => (println ! ("Criterion.rs ERROR: {}" , & format ! ($ ($ arg) *))) }
};
}
