// Generated macro for impl_60 (impl)
macro_rules! Depcrate_derive_deserialize_allowing_unknownimpl_60 {
() => {
// Module: crate::derive_deserialize_allowing_unknown
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for DeriveDeserializeAllowingUnknown { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < 'tcx >) { let ItemKind :: Impl (Impl { of_trait : Some (TraitImplHeader { trait_ref : TraitRef { path : Path { res : Res :: Def (_ , trait_def_id) , .. } , .. } , .. }) , self_ty : Ty { kind : TyKind :: Path (QPath :: Resolved (None , Path { res : Res :: Def (_ , self_ty_def_id) , .. } ,)) , .. } , .. }) = item . kind else { return ; } ; if ! paths :: SERDE_DESERIALIZE . get (cx) . contains (trait_def_id) { return ; } if ! find_attr ! (cx . tcx . get_all_attrs (item . owner_id) , AttributeKind :: AutomaticallyDerived (..)) { return ; } let Some (local_def_id) = self_ty_def_id . as_local () else { return ; } ; if ! has_variant_with_named_fields (cx . tcx , local_def_id) { return ; } let hir_id = cx . tcx . local_def_id_to_hir_id (local_def_id) ; if let Some (tokens) = find_serde_attr_item (cx . tcx , hir_id) && tokens . iter () . any (is_deny_unknown_fields_token) { return ; } span_lint (cx , DERIVE_DESERIALIZE_ALLOWING_UNKNOWN , item . span , "`#[derive(serde::Deserialize)]` without `#[serde(deny_unknown_fields)]`" ,) ; } }
};
}
