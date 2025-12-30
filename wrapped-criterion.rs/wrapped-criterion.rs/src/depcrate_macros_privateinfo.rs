// Generated macro for info (macro)
macro_rules! Depcrate_macros_privateinfo {
() => {
// Module: crate::macros_private
// Provides: {"info"}
// Dependencies: {}
# [doc = " Print a debug message to stdout. Format is the same as println! or format!"] macro_rules ! info { ($ ($ arg : tt) *) => (if $ crate :: debug_enabled () { println ! ("Criterion.rs DEBUG: {}" , & format ! ($ ($ arg) *)) }) }
};
}
