// Generated macro for get_projection_pred (function)
macro_rules! Depcrate_unit_return_expecting_ordget_projection_pred {
() => {
// Module: crate::unit_return_expecting_ord
// Provides: {"get_projection_pred"}
// Dependencies: {}
fn get_projection_pred < 'tcx > (cx : & LateContext < 'tcx > , generics : GenericPredicates < 'tcx > , trait_pred : TraitPredicate < 'tcx > ,) -> Option < ProjectionPredicate < 'tcx > > { generics . predicates . iter () . find_map (| (proj_pred , _) | { if let ClauseKind :: Projection (pred) = proj_pred . kind () . skip_binder () { let projection_pred = cx . tcx . instantiate_bound_regions_with_erased (proj_pred . kind () . rebind (pred)) ; if projection_pred . projection_term . args == trait_pred . trait_ref . args { return Some (projection_pred) ; } } None }) }
};
}
