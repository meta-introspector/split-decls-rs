// Generated macro for path (macro)
macro_rules! Depcrate_macros_privatepath {
() => {
// Module: crate::macros_private
// Provides: {"path"}
// Dependencies: {}
macro_rules ! path { ($ ($ path : tt) +) => { :: syn :: parse_quote ! ($ ($ path) +) } ; }
};
}
