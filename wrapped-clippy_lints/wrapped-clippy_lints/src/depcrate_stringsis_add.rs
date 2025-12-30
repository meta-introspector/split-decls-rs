// Generated macro for is_add (function)
macro_rules! Depcrate_stringsis_add {
() => {
// Module: crate::strings
// Provides: {"is_add"}
// Dependencies: {}
fn is_add (cx : & LateContext < '_ > , src : & Expr < '_ > , target : & Expr < '_ >) -> bool { match peel_blocks (src) . kind { ExprKind :: Binary (Spanned { node : BinOpKind :: Add , .. } , left , _ ,) => SpanlessEq :: new (cx) . eq_expr (target , left) , _ => false , } }
};
}
