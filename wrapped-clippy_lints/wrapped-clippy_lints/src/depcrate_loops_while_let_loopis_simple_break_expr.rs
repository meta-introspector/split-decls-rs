// Generated macro for is_simple_break_expr (function)
macro_rules! Depcrate_loops_while_let_loopis_simple_break_expr {
() => {
// Module: crate::loops::while_let_loop
// Provides: {"is_simple_break_expr"}
// Dependencies: {}
# [doc = " Returns `true` if expr contains a single break expression without a label or sub-expression,"] # [doc = " possibly embedded in blocks."] fn is_simple_break_expr (e : & Expr < '_ >) -> bool { if let ExprKind :: Block (b , _) = e . kind { match (b . stmts , b . expr) { ([s] , None) => matches ! (s . kind , StmtKind :: Expr (e) | StmtKind :: Semi (e) if is_simple_break_expr (e)) , ([] , Some (e)) => is_simple_break_expr (e) , _ => false , } } else { matches ! (e . kind , ExprKind :: Break (dest , None) if dest . label . is_none ()) } }
};
}
