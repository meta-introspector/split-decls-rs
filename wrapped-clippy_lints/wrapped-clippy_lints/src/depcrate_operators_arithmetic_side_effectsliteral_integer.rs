// Generated macro for literal_integer (function)
macro_rules! Depcrate_operators_arithmetic_side_effectsliteral_integer {
() => {
// Module: crate::operators::arithmetic_side_effects
// Provides: {"literal_integer"}
// Dependencies: {}
# [doc = " Returns the numeric value of a literal integer originated from `expr`, if any."] # [doc = ""] # [doc = " Literal integers can be originated from adhoc declarations like `1`, associated constants"] # [doc = " like `i32::MAX` or constant references like `N` from `const N: i32 = 1;`,"] fn literal_integer (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ >) -> Option < u128 > { let actual = peel_hir_expr_unary (expr) . 0 ; if let hir :: ExprKind :: Lit (lit) = actual . kind && let ast :: LitKind :: Int (n , _) = lit . node { return Some (n . get ()) ; } if let Some (Constant :: Int (n)) = ConstEvalCtxt :: new (cx) . eval (expr) { return Some (n) ; } None }
};
}
