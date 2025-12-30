// Generated macro for impl_9348 (impl)
macro_rules! Depcrate_redundant_localsimpl_9348 {
() => {
// Module: crate::redundant_locals
// Provides: {"impl_9348"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for RedundantLocals { fn check_local (& mut self , cx : & LateContext < 'tcx > , local : & 'tcx LetStmt < 'tcx >) { if ! local . span . is_desugaring (DesugaringKind :: Async) && let PatKind :: Binding (BindingMode (ByRef :: No , mutability) , _ , ident , None) = local . pat . kind && local . ty . is_none () && let Some (expr) = local . init && let ExprKind :: Path (qpath @ QPath :: Resolved (None , path)) = expr . kind && let [last_segment] = path . segments && last_segment . ident == ident && let Res :: Local (binding_id) = cx . qpath_res (& qpath , expr . hir_id) && let Node :: Pat (binding_pat) = cx . tcx . hir_node (binding_id) && find_binding (binding_pat , ident) . is_some_and (| bind | bind . 1 == mutability) && ! affects_assignments (cx , mutability , binding_id , local . hir_id) && ! needs_ordered_drop (cx , cx . typeck_results () . expr_ty (expr)) && ! local . span . in_external_macro (cx . sess () . source_map ()) && ! is_from_proc_macro (cx , expr) && ! is_by_value_closure_capture (cx , local . hir_id , binding_id) { span_lint_and_help (cx , REDUNDANT_LOCALS , local . span , format ! ("redundant redefinition of a binding `{ident}`") , Some (binding_pat . span) , format ! ("`{ident}` is initially defined here") ,) ; } } }
};
}
