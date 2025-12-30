// Generated macro for fetch_cloned_expr (function)
macro_rules! Depcrate_loops_manual_memcpyfetch_cloned_expr {
() => {
// Module: crate::loops::manual_memcpy
// Provides: {"fetch_cloned_expr"}
// Dependencies: {}
fn fetch_cloned_expr < 'tcx > (expr : & 'tcx Expr < 'tcx >) -> & 'tcx Expr < 'tcx > { if let ExprKind :: MethodCall (method , arg , [] , _) = expr . kind && method . ident . name == sym :: clone { arg } else { expr } }
};
}
