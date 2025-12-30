// Generated macro for impl_1556 (impl)
macro_rules! Depcrate_disallowed_methodsimpl_1556 {
() => {
// Module: crate::disallowed_methods
// Provides: {"impl_1556"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for DisallowedMethods { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { let (id , span) = match & expr . kind { ExprKind :: Path (path) if let Res :: Def (_ , id) = cx . qpath_res (path , expr . hir_id) => (id , expr . span) , ExprKind :: MethodCall (name , ..) if let Some (id) = cx . typeck_results () . type_dependent_def_id (expr . hir_id) => { (id , name . ident . span) } , _ => return , } ; if let Some (& (path , disallowed_path)) = self . disallowed . get (& id) { span_lint_and_then (cx , DISALLOWED_METHODS , span , format ! ("use of a disallowed method `{path}`") , disallowed_path . diag_amendment (span) ,) ; } } }
};
}
