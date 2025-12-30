// Generated macro for impl_210 (impl)
macro_rules! Depcrate_unusual_namesimpl_210 {
() => {
// Module: crate::unusual_names
// Provides: {"impl_210"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for UnusualNames { fn check_stmt (& mut self , cx : & LateContext < 'tcx > , stmt : & 'tcx Stmt < '_ >) { if let StmtKind :: Let (let_stmt) = stmt . kind && let Some (init_expr) = let_stmt . init { check_pat_name_for_ty (cx , let_stmt . pat , cx . typeck_results () . expr_ty (init_expr) , "variable") ; } } fn check_fn (& mut self , cx : & LateContext < 'tcx > , kind : FnKind < 'tcx > , _decl : & 'tcx FnDecl < '_ > , body : & 'tcx Body < '_ > , _span : Span , def_id : LocalDefId ,) { if matches ! (kind , FnKind :: Closure) { return ; } for (param , ty) in body . params . iter () . zip (cx . tcx . fn_sig (def_id) . instantiate_identity () . skip_binder () . inputs ()) { check_pat_name_for_ty (cx , param . pat , * ty , "parameter") ; } } }
};
}
