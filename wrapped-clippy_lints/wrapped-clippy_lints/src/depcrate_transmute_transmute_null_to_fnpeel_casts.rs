// Generated macro for peel_casts (function)
macro_rules! Depcrate_transmute_transmute_null_to_fnpeel_casts {
() => {
// Module: crate::transmute::transmute_null_to_fn
// Provides: {"peel_casts"}
// Dependencies: {}
fn peel_casts < 'tcx > (expr : & 'tcx Expr < 'tcx >) -> & 'tcx Expr < 'tcx > { match & expr . kind { ExprKind :: Cast (inner_expr , _) => peel_casts (inner_expr) , _ => expr , } }
};
}
