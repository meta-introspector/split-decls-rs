// Generated macro for as_bool_lit (function)
macro_rules! Depcrate_assertions_on_constantsas_bool_lit {
() => {
// Module: crate::assertions_on_constants
// Provides: {"as_bool_lit"}
// Dependencies: {}
fn as_bool_lit (e : & Expr < '_ >) -> Option < bool > { if let ExprKind :: Lit (l) = e . kind && let LitKind :: Bool (b) = l . node { Some (b) } else { None } }
};
}
