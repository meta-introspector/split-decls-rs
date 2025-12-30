// Generated macro for check_stmt (function)
macro_rules! Depcrate_mixed_read_write_in_expressioncheck_stmt {
() => {
// Module: crate::mixed_read_write_in_expression
// Provides: {"check_stmt"}
// Dependencies: {}
fn check_stmt < 'tcx > (vis : & mut ReadVisitor < '_ , 'tcx > , stmt : & 'tcx Stmt < '_ >) -> StopEarly { match stmt . kind { StmtKind :: Expr (expr) | StmtKind :: Semi (expr) => check_expr (vis , expr) , StmtKind :: Let (local) => local . init . as_ref () . map_or (StopEarly :: KeepGoing , | expr | check_expr (vis , expr)) , StmtKind :: Item (..) => StopEarly :: KeepGoing , } }
};
}
