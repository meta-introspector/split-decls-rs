// Generated macro for check_call_arguments (function)
macro_rules! Depcrate_loops_manual_while_let_somecheck_call_arguments {
() => {
// Module: crate::loops::manual_while_let_some
// Provides: {"check_call_arguments"}
// Dependencies: {}
fn check_call_arguments (cx : & LateContext < '_ > , stmt : & Stmt < '_ > , is_empty_recv : & Expr < '_ > , loop_span : Span) { if let StmtKind :: Semi (expr) | StmtKind :: Expr (expr) = stmt . kind && let ExprKind :: MethodCall (.. , args , _) | ExprKind :: Call (_ , args) = expr . kind { let offending_arg = args . iter () . find_map (| arg | is_vec_pop_unwrap (cx , arg , is_empty_recv) . then_some (arg . span)) ; if let Some (offending_arg) = offending_arg { report_lint (cx , offending_arg , PopStmt :: Anonymous , loop_span , is_empty_recv . span) ; } } }
};
}
