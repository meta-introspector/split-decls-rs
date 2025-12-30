// Generated macro for is_async_move_desugar (function)
macro_rules! Depcrate_dbg_macrois_async_move_desugar {
() => {
// Module: crate::dbg_macro
// Provides: {"is_async_move_desugar"}
// Dependencies: {}
fn is_async_move_desugar < 'tcx > (expr : & 'tcx Expr < 'tcx >) -> Option < & 'tcx Expr < 'tcx > > { if let ExprKind :: Block (block , _) = expr . kind && let [Stmt { kind : StmtKind :: Let (LetStmt { source : LocalSource :: AsyncFn , .. }) , .. } ,] = block . stmts { return block . expr ; } None }
};
}
