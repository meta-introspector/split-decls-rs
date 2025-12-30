// Generated macro for direct_super_trait_refs (function)
macro_rules! Depcrate_utilsdirect_super_trait_refs {
() => {
// Module: crate::utils
// Provides: {"direct_super_trait_refs"}
// Dependencies: {}
fn direct_super_trait_refs (db : & dyn HirDatabase , trait_ref : & TraitRef , cb : impl FnMut (TraitRef)) { let generic_params = db . generic_params (trait_ref . hir_trait_id () . into ()) ; let trait_self = match generic_params . trait_self_param () { Some (p) => TypeOrConstParamId { parent : trait_ref . hir_trait_id () . into () , local_id : p } , None => return , } ; db . generic_predicates_for_param (trait_self . parent , trait_self , None) . iter () . filter_map (| pred | { pred . as_ref () . filter_map (| pred | match pred . skip_binders () { WhereClause :: Implemented (tr) => Some (tr . clone () . shifted_out_to (Interner , DebruijnIndex :: ONE) . expect ("FIXME unexpected higher-ranked trait bound") ,) , _ => None , }) }) . map (| pred | pred . substitute (Interner , & trait_ref . substitution)) . for_each (cb) ; }
};
}
