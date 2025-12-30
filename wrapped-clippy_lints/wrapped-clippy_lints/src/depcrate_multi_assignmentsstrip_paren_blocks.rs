// Generated macro for strip_paren_blocks (function)
macro_rules! Depcrate_multi_assignmentsstrip_paren_blocks {
() => {
// Module: crate::multi_assignments
// Provides: {"strip_paren_blocks"}
// Dependencies: {}
fn strip_paren_blocks (expr : & Expr) -> & Expr { match & expr . kind { ExprKind :: Paren (e) => strip_paren_blocks (e) , ExprKind :: Block (b , _) => { if let [Stmt { kind : StmtKind :: Expr (e) , .. } ,] = & b . stmts [..] { strip_paren_blocks (e) } else { expr } } , _ => expr , } }
};
}
