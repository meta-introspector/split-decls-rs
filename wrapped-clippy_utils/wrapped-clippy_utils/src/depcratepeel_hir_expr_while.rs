// Generated macro for peel_hir_expr_while (function)
macro_rules! Depcratepeel_hir_expr_while {
() => {
// Module: crate
// Provides: {"peel_hir_expr_while"}
// Dependencies: {}
# [doc = " Peels of expressions while the given closure returns `Some`."] pub fn peel_hir_expr_while < 'tcx > (mut expr : & 'tcx Expr < 'tcx > , mut f : impl FnMut (& 'tcx Expr < 'tcx >) -> Option < & 'tcx Expr < 'tcx > > ,) -> & 'tcx Expr < 'tcx > { while let Some (e) = f (expr) { expr = e ; } expr }
};
}
