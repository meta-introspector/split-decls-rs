// Generated macro for impl_1307 (impl)
macro_rules! Depcrate_unit_bindingsimpl_1307 {
() => {
// Module: crate::unit_bindings
// Provides: {"impl_1307"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for UnitBindings { fn check_local (& mut self , cx : & crate :: LateContext < 'tcx > , local : & 'tcx hir :: LetStmt < 'tcx >) { if ! local . span . from_expansion () && let Some (tyck_results) = cx . maybe_typeck_results () && let Some (init) = local . init && let init_ty = tyck_results . expr_ty (init) && let local_ty = tyck_results . node_type (local . hir_id) && init_ty == cx . tcx . types . unit && local_ty == cx . tcx . types . unit && local . ty . is_none () && ! matches ! (init . kind , hir :: ExprKind :: Tup ([])) && ! matches ! (local . pat . kind , hir :: PatKind :: Tuple ([] , ..)) { cx . emit_span_lint (UNIT_BINDINGS , local . span , UnitBindingsDiag { label : local . pat . span } ,) ; } } }
};
}
