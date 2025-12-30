// Generated macro for impl_7570 (impl)
macro_rules! Depcrate_mut_referenceimpl_7570 {
() => {
// Module: crate::mut_reference
// Provides: {"impl_7570"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for UnnecessaryMutPassed { fn check_expr (& mut self , cx : & LateContext < 'tcx > , e : & 'tcx Expr < '_ >) { if e . span . from_expansion () { return ; } match e . kind { ExprKind :: Call (fn_expr , arguments) => { if let ExprKind :: Path (ref path) = fn_expr . kind { check_arguments (cx , & mut arguments . iter () , cx . typeck_results () . expr_ty (fn_expr) , & rustc_hir_pretty :: qpath_to_string (& cx . tcx , path) , "function" ,) ; } } , ExprKind :: MethodCall (path , receiver , arguments , _) if let Some (def_id) = cx . typeck_results () . type_dependent_def_id (e . hir_id) => { let args = cx . typeck_results () . node_args (e . hir_id) ; let method_type = cx . tcx . type_of (def_id) . instantiate (cx . tcx , args) ; check_arguments (cx , & mut iter :: once (receiver) . chain (arguments . iter ()) , method_type , path . ident . as_str () , "method" ,) ; } , _ => () , } } }
};
}
