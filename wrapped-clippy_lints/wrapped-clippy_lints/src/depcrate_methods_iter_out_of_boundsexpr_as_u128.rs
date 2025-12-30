// Generated macro for expr_as_u128 (function)
macro_rules! Depcrate_methods_iter_out_of_boundsexpr_as_u128 {
() => {
// Module: crate::methods::iter_out_of_bounds
// Provides: {"expr_as_u128"}
// Dependencies: {}
fn expr_as_u128 (cx : & LateContext < '_ > , e : & Expr < '_ >) -> Option < u128 > { if let ExprKind :: Lit (lit) = expr_or_init (cx , e) . kind && let LitKind :: Int (n , _) = lit . node { Some (n . get ()) } else { None } }
};
}
