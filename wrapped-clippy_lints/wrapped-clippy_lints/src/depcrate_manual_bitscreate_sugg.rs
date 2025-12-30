// Generated macro for create_sugg (function)
macro_rules! Depcrate_manual_bitscreate_sugg {
() => {
// Module: crate::manual_bits
// Provides: {"create_sugg"}
// Dependencies: {}
fn create_sugg (cx : & LateContext < '_ > , expr : & Expr < '_ > , base_sugg : String) -> String { if let Some (parent_expr) = get_parent_expr (cx , expr) { if is_ty_conversion (parent_expr) { return base_sugg ; } match parent_expr . kind { ExprKind :: Unary (..) | ExprKind :: AddrOf (..) | ExprKind :: MethodCall (..) => { return format ! ("({base_sugg} as usize)") ; } , _ => { } , } } format ! ("{base_sugg} as usize") }
};
}
