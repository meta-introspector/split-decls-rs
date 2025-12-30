// Generated macro for impl_4406 (impl)
macro_rules! Depcrate_manual_retainimpl_4406 {
() => {
// Module: crate::manual_retain
// Provides: {"impl_4406"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for ManualRetain { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < '_ >) { if let Assign (left_expr , collect_expr , _) = & expr . kind && let hir :: ExprKind :: MethodCall (seg , target_expr , [] , _) = & collect_expr . kind && seg . args . is_none () && let Some (collect_def_id) = cx . typeck_results () . type_dependent_def_id (collect_expr . hir_id) && cx . tcx . is_diagnostic_item (sym :: iterator_collect_fn , collect_def_id) { check_into_iter (cx , left_expr , target_expr , expr . span , self . msrv) ; check_iter (cx , left_expr , target_expr , expr . span , self . msrv) ; check_to_owned (cx , left_expr , target_expr , expr . span , self . msrv) ; } } }
};
}
