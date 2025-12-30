// Generated macro for size_of (function)
macro_rules! Depcrate_useless_vecsize_of {
() => {
// Module: crate::useless_vec
// Provides: {"size_of"}
// Dependencies: {}
fn size_of (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> u64 { let ty = cx . typeck_results () . expr_ty_adjusted (expr) ; cx . layout_of (ty) . map_or (0 , | l | l . size . bytes ()) }
};
}
