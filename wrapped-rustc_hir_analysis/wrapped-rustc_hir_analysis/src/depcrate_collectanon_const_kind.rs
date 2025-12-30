// Generated macro for anon_const_kind (function)
macro_rules! Depcrate_collectanon_const_kind {
() => {
// Module: crate::collect
// Provides: {"anon_const_kind"}
// Dependencies: {}
fn anon_const_kind < 'tcx > (tcx : TyCtxt < 'tcx > , def : LocalDefId) -> ty :: AnonConstKind { let hir_id = tcx . local_def_id_to_hir_id (def) ; let const_arg_id = tcx . parent_hir_id (hir_id) ; match tcx . hir_node (const_arg_id) { hir :: Node :: ConstArg (_) => { if tcx . features () . generic_const_exprs () { ty :: AnonConstKind :: GCE } else if tcx . features () . min_generic_const_args () { ty :: AnonConstKind :: MCG } else if let hir :: Node :: Expr (hir :: Expr { kind : hir :: ExprKind :: Repeat (_ , repeat_count) , .. }) = tcx . hir_node (tcx . parent_hir_id (const_arg_id)) && repeat_count . hir_id == const_arg_id { ty :: AnonConstKind :: RepeatExprCount } else { ty :: AnonConstKind :: MCG } } _ => ty :: AnonConstKind :: NonTypeSystem , } }
};
}
