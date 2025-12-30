// Generated macro for impl_1511 (impl)
macro_rules! Depcrate_deriveimpl_1511 {
() => {
// Module: crate::derive
// Provides: {"impl_1511"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for Derive { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < '_ >) { if let ItemKind :: Impl (Impl { of_trait : Some (of_trait) , .. }) = item . kind { let trait_ref = & of_trait . trait_ref ; let ty = cx . tcx . type_of (item . owner_id) . instantiate_identity () ; let is_automatically_derived = cx . tcx . is_automatically_derived (item . owner_id . to_def_id ()) ; check_hash_peq (cx , item . span , trait_ref , ty , is_automatically_derived) ; check_ord_partial_ord (cx , item . span , trait_ref , ty , is_automatically_derived) ; if is_automatically_derived { check_unsafe_derive_deserialize (cx , item , trait_ref , ty) ; check_partial_eq_without_eq (cx , item . span , trait_ref , ty) ; } else { check_copy_clone (cx , item , trait_ref , ty) ; } } } }
};
}
