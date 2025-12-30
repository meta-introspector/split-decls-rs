// Generated macro for check_expr (function)
macro_rules! Depcrate_mixed_read_write_in_expressioncheck_expr {
() => {
// Module: crate::mixed_read_write_in_expression
// Provides: {"check_expr"}
// Dependencies: {}
fn check_expr < 'tcx > (vis : & mut ReadVisitor < '_ , 'tcx > , expr : & 'tcx Expr < '_ >) -> StopEarly { if expr . hir_id == vis . last_expr . hir_id { return StopEarly :: KeepGoing ; } match expr . kind { ExprKind :: Array (_) | ExprKind :: Tup (_) | ExprKind :: MethodCall (..) | ExprKind :: Call (_ , _) | ExprKind :: Assign (..) | ExprKind :: Index (..) | ExprKind :: Repeat (_ , _) | ExprKind :: Struct (_ , _ , _) | ExprKind :: AssignOp (_ , _ , _) => { walk_expr (vis , expr) ; } , ExprKind :: Binary (op , _ , _) => { if op . node == BinOpKind :: And || op . node == BinOpKind :: Or { } else { walk_expr (vis , expr) ; } } , ExprKind :: Closure { .. } => { return StopEarly :: Stop ; } , _ => { } , } vis . last_expr = expr ; StopEarly :: KeepGoing }
};
}
