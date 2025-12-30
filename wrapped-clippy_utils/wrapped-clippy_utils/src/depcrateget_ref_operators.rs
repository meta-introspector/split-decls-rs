// Generated macro for get_ref_operators (function)
macro_rules! Depcrateget_ref_operators {
() => {
// Module: crate
// Provides: {"get_ref_operators"}
// Dependencies: {}
# [doc = " Returns a `Vec` of `Expr`s containing `AddrOf` operators (`&`) or deref operators (`*`) of a"] # [doc = " given expression."] pub fn get_ref_operators < 'hir > (cx : & LateContext < '_ > , expr : & 'hir Expr < 'hir >) -> Vec < & 'hir Expr < 'hir > > { let mut operators = Vec :: new () ; peel_hir_expr_while (expr , | expr | match expr . kind { ExprKind :: AddrOf (_ , _ , e) => { operators . push (expr) ; Some (e) } , ExprKind :: Unary (UnOp :: Deref , e) if cx . typeck_results () . expr_ty (e) . is_ref () => { operators . push (expr) ; Some (e) } , _ => None , }) ; operators }
};
}
