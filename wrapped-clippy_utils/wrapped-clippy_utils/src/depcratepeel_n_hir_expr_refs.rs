// Generated macro for peel_n_hir_expr_refs (function)
macro_rules! Depcratepeel_n_hir_expr_refs {
() => {
// Module: crate
// Provides: {"peel_n_hir_expr_refs"}
// Dependencies: {}
# [doc = " Peels off up to the given number of references on the expression. Returns the underlying"] # [doc = " expression and the number of references removed."] pub fn peel_n_hir_expr_refs < 'a > (expr : & 'a Expr < 'a > , count : usize) -> (& 'a Expr < 'a > , usize) { let mut remaining = count ; let e = peel_hir_expr_while (expr , | e | match e . kind { ExprKind :: AddrOf (ast :: BorrowKind :: Ref , _ , e) if remaining != 0 => { remaining -= 1 ; Some (e) } , _ => None , }) ; (e , count - remaining) }
};
}
