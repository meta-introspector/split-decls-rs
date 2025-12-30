// Generated macro for impl_8938 (impl)
macro_rules! Depcrate_pattern_type_mismatchimpl_8938 {
() => {
// Module: crate::pattern_type_mismatch
// Provides: {"impl_8938"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for PatternTypeMismatch { fn check_stmt (& mut self , cx : & LateContext < 'tcx > , stmt : & 'tcx Stmt < '_ >) { if let StmtKind :: Let (local) = stmt . kind { if local . pat . span . in_external_macro (cx . sess () . source_map ()) { return ; } let deref_possible = match local . source { LocalSource :: Normal => DerefPossible :: Possible , _ => DerefPossible :: Impossible , } ; apply_lint (cx , local . pat , deref_possible) ; } } fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if let ExprKind :: Match (_ , arms , _) = expr . kind { if expr . span . in_external_macro (cx . sess () . source_map ()) { return ; } for arm in arms { let pat = & arm . pat ; if apply_lint (cx , pat , DerefPossible :: Possible) { break ; } } } if let ExprKind :: Let (LetExpr { pat , .. }) = expr . kind { apply_lint (cx , pat , DerefPossible :: Possible) ; } } fn check_fn (& mut self , cx : & LateContext < 'tcx > , _ : intravisit :: FnKind < 'tcx > , _ : & 'tcx FnDecl < '_ > , body : & 'tcx Body < '_ > , _ : Span , _ : LocalDefId ,) { for param in body . params { apply_lint (cx , param . pat , DerefPossible :: Impossible) ; } } }
};
}
