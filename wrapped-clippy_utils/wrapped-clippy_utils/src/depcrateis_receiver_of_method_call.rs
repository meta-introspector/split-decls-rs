// Generated macro for is_receiver_of_method_call (function)
macro_rules! Depcrateis_receiver_of_method_call {
() => {
// Module: crate
// Provides: {"is_receiver_of_method_call"}
// Dependencies: {}
# [doc = " Returns true if the specified expression is in a receiver position."] pub fn is_receiver_of_method_call (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { if let Some (parent_expr) = get_parent_expr (cx , expr) && let ExprKind :: MethodCall (_ , receiver , ..) = parent_expr . kind && receiver . hir_id == expr . hir_id { return true ; } false }
};
}
