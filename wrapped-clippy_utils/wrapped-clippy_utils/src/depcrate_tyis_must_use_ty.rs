// Generated macro for is_must_use_ty (function)
macro_rules! Depcrate_tyis_must_use_ty {
() => {
// Module: crate::ty
// Provides: {"is_must_use_ty"}
// Dependencies: {}
pub fn is_must_use_ty < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> bool { match ty . kind () { ty :: Adt (adt , _) => find_attr ! (cx . tcx . get_all_attrs (adt . did ()) , AttributeKind :: MustUse { .. }) , ty :: Foreign (did) => find_attr ! (cx . tcx . get_all_attrs (* did) , AttributeKind :: MustUse { .. }) , ty :: Slice (ty) | ty :: Array (ty , _) | ty :: RawPtr (ty , _) | ty :: Ref (_ , ty , _) => { is_must_use_ty (cx , * ty) } , ty :: Tuple (args) => args . iter () . any (| ty | is_must_use_ty (cx , ty)) , ty :: Alias (ty :: Opaque , AliasTy { def_id , .. }) => { for (predicate , _) in cx . tcx . explicit_item_self_bounds (def_id) . skip_binder () { if let ty :: ClauseKind :: Trait (trait_predicate) = predicate . kind () . skip_binder () && find_attr ! (cx . tcx . get_all_attrs (trait_predicate . trait_ref . def_id) , AttributeKind :: MustUse { .. }) { return true ; } } false } , ty :: Dynamic (binder , _) => { for predicate in * binder { if let ty :: ExistentialPredicate :: Trait (ref trait_ref) = predicate . skip_binder () && find_attr ! (cx . tcx . get_all_attrs (trait_ref . def_id) , AttributeKind :: MustUse { .. }) { return true ; } } false } , _ => false , } }
};
}
