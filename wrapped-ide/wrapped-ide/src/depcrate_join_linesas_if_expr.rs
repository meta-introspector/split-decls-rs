// Generated macro for as_if_expr (function)
macro_rules! Depcrate_join_linesas_if_expr {
() => {
// Module: crate::join_lines
// Provides: {"as_if_expr"}
// Dependencies: {}
fn as_if_expr (element : & SyntaxElement) -> Option < ast :: IfExpr > { let mut node = element . as_node () ? . clone () ; if let Some (stmt) = ast :: ExprStmt :: cast (node . clone ()) { node = stmt . expr () ? . syntax () . clone () ; } ast :: IfExpr :: cast (node) }
};
}
