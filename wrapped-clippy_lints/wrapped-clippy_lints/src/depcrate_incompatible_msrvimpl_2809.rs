// Generated macro for impl_2809 (impl)
macro_rules! Depcrate_incompatible_msrvimpl_2809 {
() => {
// Module: crate::incompatible_msrv
// Provides: {"impl_2809"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for IncompatibleMsrv { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) { match expr . kind { ExprKind :: MethodCall (_ , _ , _ , span) => { if let Some (method_did) = cx . typeck_results () . type_dependent_def_id (expr . hir_id) { self . emit_lint_if_under_msrv (cx , method_did , expr . hir_id , span) ; } } , ExprKind :: Path (qpath @ (QPath :: Resolved (..) | QPath :: TypeRelative (..))) => { if let Some (path_def_id) = cx . qpath_res (& qpath , expr . hir_id) . opt_def_id () { self . emit_lint_if_under_msrv (cx , path_def_id , expr . hir_id , expr . span) ; } } , _ => { } , } } fn check_ty (& mut self , cx : & LateContext < 'tcx > , hir_ty : & 'tcx hir :: Ty < 'tcx , AmbigArg >) { if let hir :: TyKind :: Path (qpath @ (QPath :: Resolved (..) | QPath :: TypeRelative (..))) = hir_ty . kind && let Some (ty_def_id) = cx . qpath_res (& qpath , hir_ty . hir_id) . opt_def_id () && ! matches ! (cx . tcx . get_diagnostic_name (ty_def_id) , Some (sym :: cstr_type | sym :: cstring_type)) { self . emit_lint_if_under_msrv (cx , ty_def_id , hir_ty . hir_id , hir_ty . span) ; } } }
};
}
