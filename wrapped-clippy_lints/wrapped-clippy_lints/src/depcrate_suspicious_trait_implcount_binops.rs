// Generated macro for count_binops (function)
macro_rules! Depcrate_suspicious_trait_implcount_binops {
() => {
// Module: crate::suspicious_trait_impl
// Provides: {"count_binops"}
// Dependencies: {}
fn count_binops (expr : & hir :: Expr < '_ >) -> u32 { let mut count = 0u32 ; let _ : Option < ! > = for_each_expr_without_closures (expr , | e | { if matches ! (e . kind , hir :: ExprKind :: Binary (..) | hir :: ExprKind :: Unary (hir :: UnOp :: Not | hir :: UnOp :: Neg , _) | hir :: ExprKind :: AssignOp (..)) { count += 1 ; } ControlFlow :: Continue (()) }) ; count }
};
}
