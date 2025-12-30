// Generated macro for as_int_bool_lit (function)
macro_rules! Depcrate_bool_to_int_with_ifas_int_bool_lit {
() => {
// Module: crate::bool_to_int_with_if
// Provides: {"as_int_bool_lit"}
// Dependencies: {}
fn as_int_bool_lit (expr : & Expr < '_ >) -> Option < bool > { if let ExprKind :: Block (b , _) = expr . kind && b . stmts . is_empty () && let Some (e) = b . expr && ! e . span . from_expansion () && let ExprKind :: Lit (lit) = e . kind && let LitKind :: Int (x , _) = lit . node { match x . get () { 0 => Some (false) , 1 => Some (true) , _ => None , } } else { None } }
};
}
