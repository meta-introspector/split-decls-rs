// Generated macro for impl_3015 (impl)
macro_rules! Depcrate_int_plus_oneimpl_3015 {
() => {
// Module: crate::int_plus_one
// Provides: {"impl_3015"}
// Dependencies: {}
impl EarlyLintPass for IntPlusOne { fn check_expr (& mut self , cx : & EarlyContext < '_ > , item : & Expr) { if let ExprKind :: Binary (ref kind , ref lhs , ref rhs) = item . kind && let Some (rec) = Self :: check_binop (cx , kind . node , lhs , rhs) { Self :: emit_warning (cx , item , rec) ; } } }
};
}
