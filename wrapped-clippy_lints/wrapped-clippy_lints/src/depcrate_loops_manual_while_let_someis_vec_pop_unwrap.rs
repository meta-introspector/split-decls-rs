// Generated macro for is_vec_pop_unwrap (function)
macro_rules! Depcrate_loops_manual_while_let_someis_vec_pop_unwrap {
() => {
// Module: crate::loops::manual_while_let_some
// Provides: {"is_vec_pop_unwrap"}
// Dependencies: {}
fn is_vec_pop_unwrap (cx : & LateContext < '_ > , expr : & Expr < '_ > , is_empty_recv : & Expr < '_ >) -> bool { if (match_method_call :: < 0 > (cx , expr , sym :: option_unwrap) || match_method_call :: < 1 > (cx , expr , sym :: option_expect)) && let ExprKind :: MethodCall (_ , unwrap_recv , ..) = expr . kind && match_method_call :: < 0 > (cx , unwrap_recv , sym :: vec_pop) && let ExprKind :: MethodCall (_ , pop_recv , ..) = unwrap_recv . kind { SpanlessEq :: new (cx) . eq_expr (pop_recv , is_empty_recv) } else { false } }
};
}
