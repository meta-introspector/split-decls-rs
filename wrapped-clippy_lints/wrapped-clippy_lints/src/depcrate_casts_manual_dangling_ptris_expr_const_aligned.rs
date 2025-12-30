// Generated macro for is_expr_const_aligned (function)
macro_rules! Depcrate_casts_manual_dangling_ptris_expr_const_aligned {
() => {
// Module: crate::casts::manual_dangling_ptr
// Provides: {"is_expr_const_aligned"}
// Dependencies: {}
fn is_expr_const_aligned (cx : & LateContext < '_ > , expr : & Expr < '_ > , to : & Ty < '_ >) -> bool { match expr . kind { ExprKind :: Call (fun , _) => is_align_of_call (cx , fun , to) , ExprKind :: Lit (lit) => is_literal_aligned (cx , & lit , to) , _ => false , } }
};
}
