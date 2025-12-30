// Generated macro for recursion_depth (macro)
macro_rules! Depcrate_parse_executablerecursion_depth {
() => {
// Module: crate::parse::executable
// Provides: {"recursion_depth"}
// Dependencies: {}
macro_rules ! recursion_depth { ($ remaining_depth : ident) => { { if $ remaining_depth == 0 { return Err (Error :: RecursionLimitExceeded) ; } $ remaining_depth - 1 } } ; }
};
}
