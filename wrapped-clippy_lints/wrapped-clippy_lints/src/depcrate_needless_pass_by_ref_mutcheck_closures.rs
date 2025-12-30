// Generated macro for check_closures (function)
macro_rules! Depcrate_needless_pass_by_ref_mutcheck_closures {
() => {
// Module: crate::needless_pass_by_ref_mut
// Provides: {"check_closures"}
// Dependencies: {}
fn check_closures < 'tcx > (ctx : & mut MutablyUsedVariablesCtxt < 'tcx > , cx : & LateContext < 'tcx > , checked_closures : & mut FxHashSet < LocalDefId > , closures : FxIndexSet < LocalDefId > ,) { for closure in closures { if ! checked_closures . insert (closure) { continue ; } ctx . prev_bind = None ; ctx . prev_move_to_closure . clear () ; if let Some (body) = cx . tcx . hir_node_by_def_id (closure) . associated_body () . map (| (_ , body_id) | cx . tcx . hir_body (body_id)) { euv :: ExprUseVisitor :: for_clippy (cx , closure , & mut * ctx) . consume_body (body) . into_ok () ; } } }
};
}
