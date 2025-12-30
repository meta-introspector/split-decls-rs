// Generated macro for is_reserve (function)
macro_rules! Depcrate_uninit_vecis_reserve {
() => {
// Module: crate::uninit_vec
// Provides: {"is_reserve"}
// Dependencies: {}
fn is_reserve (cx : & LateContext < '_ > , path : & PathSegment < '_ > , self_expr : & Expr < '_ >) -> bool { cx . typeck_results () . expr_ty (self_expr) . peel_refs () . is_diag_item (cx , sym :: Vec) && path . ident . name == sym :: reserve }
};
}
