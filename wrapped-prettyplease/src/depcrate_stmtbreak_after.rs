// Generated macro for break_after (function)
macro_rules! Depcrate_stmtbreak_after {
() => {
// Module: crate::stmt
// Provides: {"break_after"}
// Dependencies: {}
pub fn break_after (expr : & Expr) -> bool { if let Expr :: Group (group) = expr { if let Expr :: Verbatim (verbatim) = group . expr . as_ref () { return ! verbatim . is_empty () ; } } true }
};
}
