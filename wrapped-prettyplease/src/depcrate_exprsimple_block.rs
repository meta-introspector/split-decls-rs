// Generated macro for simple_block (function)
macro_rules! Depcrate_exprsimple_block {
() => {
// Module: crate::expr
// Provides: {"simple_block"}
// Dependencies: {}
pub fn simple_block (expr : & Expr) -> Option < & ExprBlock > { if let Expr :: Block (expr) = expr { if expr . attrs . is_empty () && expr . label . is_none () { return Some (expr) ; } } None }
};
}
