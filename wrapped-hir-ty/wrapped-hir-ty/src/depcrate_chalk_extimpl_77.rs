// Generated macro for impl_77 (impl)
macro_rules! Depcrate_chalk_extimpl_77 {
() => {
// Module: crate::chalk_ext
// Provides: {"impl_77"}
// Dependencies: {}
impl DynTyExt for DynTy { fn principal (& self) -> Option < Binders < Binders < & TraitRef > > > { self . bounds . as_ref () . filter_map (| bounds | { bounds . interned () . first () . and_then (| b | { b . as_ref () . filter_map (| b | match b { crate :: WhereClause :: Implemented (trait_ref) => Some (trait_ref) , _ => None , }) }) }) } fn principal_id (& self) -> Option < chalk_ir :: TraitId < Interner > > { self . bounds . skip_binders () . interned () . first () . and_then (| b | match b . skip_binders () { crate :: WhereClause :: Implemented (trait_ref) => Some (trait_ref . trait_id) , _ => None , }) } }
};
}
