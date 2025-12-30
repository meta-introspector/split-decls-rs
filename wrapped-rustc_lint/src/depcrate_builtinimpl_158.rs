// Generated macro for impl_158 (impl)
macro_rules! Depcrate_builtinimpl_158 {
() => {
// Module: crate::builtin
// Provides: {"impl_158"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for DerefNullPtr { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & hir :: Expr < '_ >) { # [doc = " test if expression is a null ptr"] fn is_null_ptr (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ >) -> bool { match & expr . kind { hir :: ExprKind :: Cast (expr , ty) => { if let hir :: TyKind :: Ptr (_) = ty . kind { return is_zero (expr) || is_null_ptr (cx , expr) ; } } hir :: ExprKind :: Call (path , _) => { if let hir :: ExprKind :: Path (ref qpath) = path . kind && let Some (def_id) = cx . qpath_res (qpath , path . hir_id) . opt_def_id () { return matches ! (cx . tcx . get_diagnostic_name (def_id) , Some (sym :: ptr_null | sym :: ptr_null_mut)) ; } } _ => { } } false } # [doc = " test if expression is the literal `0`"] fn is_zero (expr : & hir :: Expr < '_ >) -> bool { match & expr . kind { hir :: ExprKind :: Lit (lit) => { if let LitKind :: Int (a , _) = lit . node { return a == 0 ; } } _ => { } } false } if let hir :: ExprKind :: Unary (hir :: UnOp :: Deref , expr_deref) = expr . kind && is_null_ptr (cx , expr_deref) { if let hir :: Node :: Expr (hir :: Expr { kind : hir :: ExprKind :: AddrOf (hir :: BorrowKind :: Raw , ..) , .. }) = cx . tcx . parent_hir_node (expr . hir_id) { } else { cx . emit_span_lint (DEREF_NULLPTR , expr . span , BuiltinDerefNullptr { label : expr . span } ,) ; } } } }
};
}
