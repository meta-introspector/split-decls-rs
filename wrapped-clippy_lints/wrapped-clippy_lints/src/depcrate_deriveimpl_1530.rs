// Generated macro for impl_1530 (impl)
macro_rules! Depcrate_deriveimpl_1530 {
() => {
// Module: crate::derive
// Provides: {"impl_1530"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for Derive { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < '_ >) { if let ItemKind :: Impl (Impl { of_trait : Some (of_trait) , self_ty , .. }) = item . kind && let Res :: Def (_ , def_id) = * self_ty . basic_res () && let Some (local_def_id) = def_id . as_local () { let adt_hir_id = cx . tcx . local_def_id_to_hir_id (local_def_id) ; let trait_ref = & of_trait . trait_ref ; let ty = cx . tcx . type_of (item . owner_id) . instantiate_identity () ; let is_automatically_derived = cx . tcx . is_automatically_derived (item . owner_id . to_def_id ()) ; derived_hash_with_manual_eq :: check (cx , item . span , trait_ref , ty , adt_hir_id , is_automatically_derived) ; derive_ord_xor_partial_ord :: check (cx , item . span , trait_ref , ty , adt_hir_id , is_automatically_derived) ; if is_automatically_derived { unsafe_derive_deserialize :: check (cx , item , trait_ref , ty , adt_hir_id) ; derive_partial_eq_without_eq :: check (cx , item . span , trait_ref , ty , adt_hir_id) ; } else { expl_impl_clone_on_copy :: check (cx , item , trait_ref , ty , adt_hir_id) ; } } } }
};
}
