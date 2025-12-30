// Generated macro for is_vec_or_array (function)
macro_rules! Depcrate_methods_iter_next_sliceis_vec_or_array {
() => {
// Module: crate::methods::iter_next_slice
// Provides: {"is_vec_or_array"}
// Dependencies: {}
fn is_vec_or_array < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < '_ >) -> bool { cx . typeck_results () . expr_ty (expr) . is_diag_item (cx , sym :: Vec) || matches ! (& cx . typeck_results () . expr_ty (expr) . peel_refs () . kind () , ty :: Array (_ , _)) }
};
}
