// Generated macro for inferred_outlives_of (function)
macro_rules! Depcrate_outlivesinferred_outlives_of {
() => {
// Module: crate::outlives
// Provides: {"inferred_outlives_of"}
// Dependencies: {}
pub (super) fn inferred_outlives_of (tcx : TyCtxt < '_ > , item_def_id : LocalDefId ,) -> & [(ty :: Clause < '_ > , Span)] { match tcx . def_kind (item_def_id) { DefKind :: Struct | DefKind :: Enum | DefKind :: Union => { let crate_map = tcx . inferred_outlives_crate (()) ; crate_map . predicates . get (& item_def_id . to_def_id ()) . copied () . unwrap_or (& []) } DefKind :: TyAlias if tcx . type_alias_is_lazy (item_def_id) => { let crate_map = tcx . inferred_outlives_crate (()) ; crate_map . predicates . get (& item_def_id . to_def_id ()) . copied () . unwrap_or (& []) } DefKind :: AnonConst if tcx . features () . generic_const_exprs () => { let id = tcx . local_def_id_to_hir_id (item_def_id) ; if tcx . hir_opt_const_param_default_param_def_id (id) . is_some () { let item_def_id = tcx . hir_get_parent_item (id) ; tcx . inferred_outlives_of (item_def_id) } else { & [] } } _ => & [] , } }
};
}
