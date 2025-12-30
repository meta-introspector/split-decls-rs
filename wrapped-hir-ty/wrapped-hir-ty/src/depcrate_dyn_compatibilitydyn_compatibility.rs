// Generated macro for dyn_compatibility (function)
macro_rules! Depcrate_dyn_compatibilitydyn_compatibility {
() => {
// Module: crate::dyn_compatibility
// Provides: {"dyn_compatibility"}
// Dependencies: {}
pub fn dyn_compatibility (db : & dyn HirDatabase , trait_ : TraitId ,) -> Option < DynCompatibilityViolation > { let interner = DbInterner :: new_with (db , Some (trait_ . krate (db)) , None) ; for super_trait in elaborate :: supertrait_def_ids (interner , trait_ . into ()) { if let Some (v) = db . dyn_compatibility_of_trait (super_trait . 0) { return if super_trait . 0 == trait_ { Some (v) } else { Some (DynCompatibilityViolation :: HasNonCompatibleSuperTrait (super_trait . 0)) } ; } } None }
};
}
