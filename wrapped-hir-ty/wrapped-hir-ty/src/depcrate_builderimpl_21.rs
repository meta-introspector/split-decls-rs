// Generated macro for impl_21 (impl)
macro_rules! Depcrate_builderimpl_21 {
() => {
// Module: crate::builder
// Provides: {"impl_21"}
// Dependencies: {}
impl TyBuilder < TraitId > { pub fn trait_ref (db : & dyn HirDatabase , def : TraitId) -> TyBuilder < TraitId > { TyBuilder :: subst_for_def (db , def , None) . with_data (def) } pub fn build (self) -> TraitRef { let (trait_id , substitution) = self . build_internal () ; TraitRef { trait_id : to_chalk_trait_id (trait_id) , substitution } } }
};
}
