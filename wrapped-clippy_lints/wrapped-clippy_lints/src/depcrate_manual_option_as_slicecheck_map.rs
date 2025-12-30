// Generated macro for check_map (function)
macro_rules! Depcrate_manual_option_as_slicecheck_map {
() => {
// Module: crate::manual_option_as_slice
// Provides: {"check_map"}
// Dependencies: {}
fn check_map (cx : & LateContext < '_ > , map : & Expr < '_ > , span : Span , msrv : Msrv) { if let ExprKind :: MethodCall (seg , callee , [mapping] , _) = map . kind && seg . ident . name == sym :: map && is_slice_from_ref (cx , mapping) { check_as_ref (cx , callee , span , msrv) ; } }
};
}
