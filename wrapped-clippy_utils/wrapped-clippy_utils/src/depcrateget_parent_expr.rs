// Generated macro for get_parent_expr (function)
macro_rules! Depcrateget_parent_expr {
() => {
// Module: crate
// Provides: {"get_parent_expr"}
// Dependencies: {}
# [doc = " Gets the parent expression, if any –- this is useful to constrain a lint."] pub fn get_parent_expr < 'tcx > (cx : & LateContext < 'tcx > , e : & Expr < '_ >) -> Option < & 'tcx Expr < 'tcx > > { get_parent_expr_for_hir (cx , e . hir_id) }
};
}
