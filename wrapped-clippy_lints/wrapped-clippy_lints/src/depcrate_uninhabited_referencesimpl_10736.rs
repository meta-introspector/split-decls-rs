// Generated macro for impl_10736 (impl)
macro_rules! Depcrate_uninhabited_referencesimpl_10736 {
() => {
// Module: crate::uninhabited_references
// Provides: {"impl_10736"}
// Dependencies: {}
impl LateLintPass < '_ > for UninhabitedReferences { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & '_ Expr < '_ >) { if expr . span . in_external_macro (cx . tcx . sess . source_map ()) { return ; } if let ExprKind :: Unary (UnOp :: Deref , _) = expr . kind { let ty = cx . typeck_results () . expr_ty_adjusted (expr) ; if ty . is_privately_uninhabited (cx . tcx , cx . typing_env ()) { span_lint (cx , UNINHABITED_REFERENCES , expr . span , "dereferencing a reference to an uninhabited type is undefined behavior" ,) ; } } } fn check_fn < 'tcx > (& mut self , cx : & LateContext < 'tcx > , kind : FnKind < '_ > , fndecl : & '_ FnDecl < 'tcx > , _ : & '_ Body < '_ > , span : Span , _ : LocalDefId ,) { if span . in_external_macro (cx . tcx . sess . source_map ()) || matches ! (kind , FnKind :: Closure) { return ; } if let FnRetTy :: Return (hir_ty) = fndecl . output && let TyKind :: Ref (_ , mut_ty) = hir_ty . kind && lower_ty (cx . tcx , mut_ty . ty) . is_privately_uninhabited (cx . tcx , cx . typing_env ()) { span_lint (cx , UNINHABITED_REFERENCES , hir_ty . span , "dereferencing a reference to an uninhabited type would be undefined behavior" ,) ; } } }
};
}
