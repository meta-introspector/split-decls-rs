// Generated macro for peel_hir_expr_refs (function)
macro_rules! Depcratepeel_hir_expr_refs {
() => {
// Module: crate
// Provides: {"peel_hir_expr_refs"}
// Dependencies: {}
# [doc = " Peels off all references on the expression. Returns the underlying expression and the number of"] # [doc = " references removed."] pub fn peel_hir_expr_refs < 'a > (expr : & 'a Expr < 'a >) -> (& 'a Expr < 'a > , usize) { let mut count = 0 ; let e = peel_hir_expr_while (expr , | e | match e . kind { ExprKind :: AddrOf (ast :: BorrowKind :: Ref , _ , e) => { count += 1 ; Some (e) } , _ => None , }) ; (e , count) }
};
}
