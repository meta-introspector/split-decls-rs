// Generated macro for unwrap_trivial_block (function)
macro_rules! Depcrate_utilsunwrap_trivial_block {
() => {
// Module: crate::utils
// Provides: {"unwrap_trivial_block"}
// Dependencies: {}
pub (crate) fn unwrap_trivial_block (block_expr : ast :: BlockExpr) -> ast :: Expr { extract_trivial_expression (& block_expr) . filter (| expr | ! expr . syntax () . text () . contains_char ('\n')) . unwrap_or_else (| | block_expr . into ()) }
};
}
