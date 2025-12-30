// Generated macro for peel_ref_operators (function)
macro_rules! Depcratepeel_ref_operators {
() => {
// Module: crate
// Provides: {"peel_ref_operators"}
// Dependencies: {}
# [doc = " Removes `AddrOf` operators (`&`) or deref operators (`*`), but only if a reference type is"] # [doc = " dereferenced. An overloaded deref such as `Vec` to slice would not be removed."] pub fn peel_ref_operators < 'hir > (cx : & LateContext < '_ > , mut expr : & 'hir Expr < 'hir >) -> & 'hir Expr < 'hir > { loop { match expr . kind { ExprKind :: AddrOf (_ , _ , e) => expr = e , ExprKind :: Unary (UnOp :: Deref , e) if cx . typeck_results () . expr_ty (e) . is_ref () => expr = e , _ => break , } } expr }
};
}
