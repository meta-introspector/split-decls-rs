// Generated macro for get_trait_predicates_for_trait_ids (function)
macro_rules! Depcrate_unit_return_expecting_ordget_trait_predicates_for_trait_ids {
() => {
// Module: crate::unit_return_expecting_ord
// Provides: {"get_trait_predicates_for_trait_ids"}
// Dependencies: {}
fn get_trait_predicates_for_trait_ids < 'tcx > (cx : & LateContext < 'tcx > , generics : GenericPredicates < 'tcx > , trait_ids : & [Option < DefId >] ,) -> [Vec < TraitPredicate < 'tcx > > ; 3] { debug_assert ! (trait_ids . len () >= 2) ; let mut preds = [Vec :: new () , Vec :: new () , Vec :: new ()] ; for (pred , _) in generics . predicates { if let ClauseKind :: Trait (poly_trait_pred) = pred . kind () . skip_binder () { let trait_pred = cx . tcx . instantiate_bound_regions_with_erased (pred . kind () . rebind (poly_trait_pred)) ; for (i , tid) in trait_ids . iter () . enumerate () { if let Some (tid) = tid && * tid == trait_pred . trait_ref . def_id { preds [i] . push (trait_pred) ; } } } } preds }
};
}
