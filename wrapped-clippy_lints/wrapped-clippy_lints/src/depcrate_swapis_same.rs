// Generated macro for is_same (function)
macro_rules! Depcrate_swapis_same {
() => {
// Module: crate::swap
// Provides: {"is_same"}
// Dependencies: {}
fn is_same (cx : & LateContext < '_ > , lhs : ExprOrIdent < '_ > , rhs : & Expr < '_ >) -> bool { match lhs { ExprOrIdent :: Expr (expr) => eq_expr_value (cx , expr , rhs) , ExprOrIdent :: Ident (ident) => { if let ExprKind :: Path (QPath :: Resolved (None , path)) = rhs . kind && let [segment] = & path . segments && segment . ident == ident { true } else { false } } , } }
};
}
