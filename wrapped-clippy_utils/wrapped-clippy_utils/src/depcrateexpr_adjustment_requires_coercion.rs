// Generated macro for expr_adjustment_requires_coercion (function)
macro_rules! Depcrateexpr_adjustment_requires_coercion {
() => {
// Module: crate
// Provides: {"expr_adjustment_requires_coercion"}
// Dependencies: {}
# [doc = " Checks if the expression has adjustments that require coercion, for example: dereferencing with"] # [doc = " overloaded deref, coercing pointers and `dyn` objects."] pub fn expr_adjustment_requires_coercion (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { cx . typeck_results () . expr_adjustments (expr) . iter () . any (| adj | { matches ! (adj . kind , Adjust :: Deref (Some (_)) | Adjust :: Pointer (PointerCoercion :: Unsize) | Adjust :: NeverToAny) }) }
};
}
