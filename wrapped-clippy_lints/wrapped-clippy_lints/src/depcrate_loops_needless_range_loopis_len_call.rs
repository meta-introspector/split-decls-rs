// Generated macro for is_len_call (function)
macro_rules! Depcrate_loops_needless_range_loopis_len_call {
() => {
// Module: crate::loops::needless_range_loop
// Provides: {"is_len_call"}
// Dependencies: {}
fn is_len_call (expr : & Expr < '_ > , var : Symbol) -> bool { if let ExprKind :: MethodCall (method , recv , [] , _) = expr . kind && method . ident . name == sym :: len && let ExprKind :: Path (QPath :: Resolved (_ , path)) = recv . kind && path . segments . len () == 1 && path . segments [0] . ident . name == var { return true ; } false }
};
}
