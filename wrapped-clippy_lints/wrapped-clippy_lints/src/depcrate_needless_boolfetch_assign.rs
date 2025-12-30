// Generated macro for fetch_assign (function)
macro_rules! Depcrate_needless_boolfetch_assign {
() => {
// Module: crate::needless_bool
// Provides: {"fetch_assign"}
// Dependencies: {}
fn fetch_assign < 'tcx > (expr : & 'tcx Expr < 'tcx >) -> Option < (& 'tcx Expr < 'tcx > , bool) > { if let ExprKind :: Assign (lhs , rhs , _) = peel_blocks_with_stmt (expr) . kind { fetch_bool_expr (rhs) . map (| b | (lhs , b)) } else { None } }
};
}
