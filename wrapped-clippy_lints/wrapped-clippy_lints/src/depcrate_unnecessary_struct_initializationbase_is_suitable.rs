// Generated macro for base_is_suitable (function)
macro_rules! Depcrate_unnecessary_struct_initializationbase_is_suitable {
() => {
// Module: crate::unnecessary_struct_initialization
// Provides: {"base_is_suitable"}
// Dependencies: {}
fn base_is_suitable (cx : & LateContext < '_ > , expr : & Expr < '_ > , base : & Expr < '_ >) -> bool { if ! check_references (cx , expr , base) { return false ; } if let ExprKind :: Unary (UnOp :: Deref , target) = base . kind && matches ! (target . kind , ExprKind :: Path (..)) && ! is_copy (cx , cx . typeck_results () . expr_ty (expr)) { return false ; } true }
};
}
