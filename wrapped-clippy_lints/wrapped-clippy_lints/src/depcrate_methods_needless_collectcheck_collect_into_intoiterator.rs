// Generated macro for check_collect_into_intoiterator (function)
macro_rules! Depcrate_methods_needless_collectcheck_collect_into_intoiterator {
() => {
// Module: crate::methods::needless_collect
// Provides: {"check_collect_into_intoiterator"}
// Dependencies: {}
# [doc = " checks for collecting into a (generic) method or function argument"] # [doc = " taking an `IntoIterator`"] fn check_collect_into_intoiterator < 'tcx > (cx : & LateContext < 'tcx > , parent : & 'tcx Expr < 'tcx > , collect_expr : & 'tcx Expr < 'tcx > , call_span : Span , iter_expr : & 'tcx Expr < 'tcx > ,) { if let Some (id) = fn_def_id (cx , parent) { let args = match parent . kind { ExprKind :: Call (_ , args) | ExprKind :: MethodCall (_ , _ , args , _) => args , _ => & [] , } ; if let Some (arg_idx) = args . iter () . position (| e | e . hir_id == collect_expr . hir_id) . map (| i | { if matches ! (parent . kind , ExprKind :: MethodCall (_ , _ , _ , _)) { i + 1 } else { i } }) { let inputs = cx . tcx . liberate_late_bound_regions (id , cx . tcx . fn_sig (id) . instantiate_identity ()) . inputs () ; if cx . tcx . param_env (id) . caller_bounds () . into_iter () . filter_map (| p | { if let ClauseKind :: Trait (t) = p . kind () . skip_binder () && cx . tcx . is_diagnostic_item (sym :: IntoIterator , t . trait_ref . def_id) { Some (t . self_ty ()) } else { None } }) . any (| ty | ty == inputs [arg_idx]) { span_lint_and_sugg (cx , NEEDLESS_COLLECT , call_span . with_lo (iter_expr . span . hi ()) , NEEDLESS_COLLECT_MSG , "remove this call" , String :: new () , Applicability :: MachineApplicable ,) ; } } } }
};
}
