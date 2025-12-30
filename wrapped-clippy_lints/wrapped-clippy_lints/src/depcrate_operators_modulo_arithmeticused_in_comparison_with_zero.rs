// Generated macro for used_in_comparison_with_zero (function)
macro_rules! Depcrate_operators_modulo_arithmeticused_in_comparison_with_zero {
() => {
// Module: crate::operators::modulo_arithmetic
// Provides: {"used_in_comparison_with_zero"}
// Dependencies: {}
fn used_in_comparison_with_zero (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { if let Node :: Expr (parent_expr) = cx . tcx . parent_hir_node (expr . hir_id) && let ExprKind :: Binary (op , lhs , rhs) = parent_expr . kind && let BinOpKind :: Eq | BinOpKind :: Ne = op . node { let ecx = ConstEvalCtxt :: new (cx) ; let ctxt = expr . span . ctxt () ; matches ! (ecx . eval_local (lhs , ctxt) , Some (Constant :: Int (0))) || matches ! (ecx . eval_local (rhs , ctxt) , Some (Constant :: Int (0))) } else { false } }
};
}
