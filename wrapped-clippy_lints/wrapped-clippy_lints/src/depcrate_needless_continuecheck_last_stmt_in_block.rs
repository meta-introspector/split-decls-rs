// Generated macro for check_last_stmt_in_block (function)
macro_rules! Depcrate_needless_continuecheck_last_stmt_in_block {
() => {
// Module: crate::needless_continue
// Provides: {"check_last_stmt_in_block"}
// Dependencies: {}
fn check_last_stmt_in_block < F > (cx : & LateContext < '_ > , b : & Block < '_ > , func : & F) where F : Fn (Option < & Label > , Span) , { if let Some (expr) = b . expr { check_last_stmt_in_expr (cx , expr , func) ; return ; } if let Some (last_stmt) = b . stmts . last () && let StmtKind :: Expr (inner_expr) | StmtKind :: Semi (inner_expr) = last_stmt . kind { check_last_stmt_in_expr (cx , inner_expr , func) ; } }
};
}
