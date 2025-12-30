// Generated macro for space (macro)
macro_rules! Depcrate_macrosspace {
() => {
// Module: crate::macros
// Provides: {"space"}
// Dependencies: {}
macro_rules ! space { ($ bytes : ident or $ err : expr) => ({ expect ! ($ bytes . next () == b' ' => Err ($ err)) ; $ bytes . slice () ; }) }
};
}
