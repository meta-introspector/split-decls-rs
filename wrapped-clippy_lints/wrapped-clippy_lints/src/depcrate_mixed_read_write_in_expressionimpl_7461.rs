// Generated macro for impl_7461 (impl)
macro_rules! Depcrate_mixed_read_write_in_expressionimpl_7461 {
() => {
// Module: crate::mixed_read_write_in_expression
// Provides: {"impl_7461"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for EvalOrderDependence { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { let var = if let ExprKind :: Assign (lhs , ..) | ExprKind :: AssignOp (_ , lhs , _) = expr . kind && let Some (var) = path_to_local (lhs) && expr . span . desugaring_kind () . is_none () { var } else { return ; } ; let mut visitor = ReadVisitor { cx , var , write_expr : expr , last_expr : expr , } ; check_for_unsequenced_reads (& mut visitor) ; } fn check_stmt (& mut self , cx : & LateContext < 'tcx > , stmt : & 'tcx Stmt < '_ >) { match stmt . kind { StmtKind :: Let (local) => { if let LetStmt { init : Some (e) , .. } = local { DivergenceVisitor { cx } . visit_expr (e) ; } } , StmtKind :: Expr (e) | StmtKind :: Semi (e) => DivergenceVisitor { cx } . maybe_walk_expr (e) , StmtKind :: Item (..) => { } , } } }
};
}
