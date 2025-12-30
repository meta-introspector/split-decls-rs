// Generated macro for newline (macro)
macro_rules! Depcrate_macrosnewline {
() => {
// Module: crate::macros
// Provides: {"newline"}
// Dependencies: {}
macro_rules ! newline { ($ bytes : ident) => ({ match next ! ($ bytes) { b'\r' => { expect ! ($ bytes . next () == b'\n' => Err (Error :: NewLine)) ; $ bytes . slice () ; } , b'\n' => { $ bytes . slice () ; } , _ => return Err (Error :: NewLine) } }) }
};
}
