// Generated macro for impl_51 (impl)
macro_rules! Depcrate_chalk_dbimpl_51 {
() => {
// Module: crate::chalk_db
// Provides: {"impl_51"}
// Dependencies: {}
impl ChalkContext < '_ > { fn edition (& self) -> Edition { self . krate . data (self . db) . edition } fn for_trait_impls (& self , trait_id : hir_def :: TraitId , self_ty_fp : Option < TyFingerprint > , mut f : impl FnMut (& TraitImpls) -> ControlFlow < () > ,) -> ControlFlow < () > { let in_deps = self . db . trait_impls_in_deps (self . krate) ; let in_self = self . db . trait_impls_in_crate (self . krate) ; let trait_module = trait_id . module (self . db) ; let type_module = match self_ty_fp { Some (TyFingerprint :: Adt (adt_id)) => Some (adt_id . module (self . db)) , Some (TyFingerprint :: ForeignType (type_id)) => { Some (from_foreign_def_id (type_id) . module (self . db)) } Some (TyFingerprint :: Dyn (trait_id)) => Some (trait_id . module (self . db)) , _ => None , } ; let mut def_blocks = [trait_module . containing_block () , type_module . and_then (| it | it . containing_block ())] ; let block_impls = iter :: successors (self . block , | & block_id | { cov_mark :: hit ! (block_local_impls) ; block_id . loc (self . db) . module . containing_block () }) . inspect (| & block_id | { def_blocks . iter_mut () . for_each (| block | { if * block == Some (block_id) { * block = None ; } }) ; }) . filter_map (| block_id | self . db . trait_impls_in_block (block_id)) ; f (& in_self) ? ; for it in in_deps . iter () . map (ops :: Deref :: deref) { f (it) ? ; } for it in block_impls { f (& it) ? ; } for it in def_blocks . into_iter () . flatten () . filter_map (| it | self . db . trait_impls_in_block (it)) { f (& it) ? ; } ControlFlow :: Continue (()) } }
};
}
