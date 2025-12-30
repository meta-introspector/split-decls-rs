// Generated macro for check_local (function)
macro_rules! Depcrate_loops_manual_while_let_somecheck_local {
() => {
// Module: crate::loops::manual_while_let_some
// Provides: {"check_local"}
// Dependencies: {}
fn check_local (cx : & LateContext < '_ > , stmt : & Stmt < '_ > , is_empty_recv : & Expr < '_ > , loop_span : Span) { if let StmtKind :: Let (local) = stmt . kind && let Some (init) = local . init && is_vec_pop_unwrap (cx , init , is_empty_recv) { report_lint (cx , stmt . span , PopStmt :: Local (local . pat) , loop_span , is_empty_recv . span) ; } }
};
}
