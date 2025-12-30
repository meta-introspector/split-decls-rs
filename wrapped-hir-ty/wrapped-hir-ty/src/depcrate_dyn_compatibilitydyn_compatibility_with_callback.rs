// Generated macro for dyn_compatibility_with_callback (function)
macro_rules! Depcrate_dyn_compatibilitydyn_compatibility_with_callback {
() => {
// Module: crate::dyn_compatibility
// Provides: {"dyn_compatibility_with_callback"}
// Dependencies: {}
pub fn dyn_compatibility_with_callback < F > (db : & dyn HirDatabase , trait_ : TraitId , cb : & mut F ,) -> ControlFlow < () > where F : FnMut (DynCompatibilityViolation) -> ControlFlow < () > , { let interner = DbInterner :: new_with (db , Some (trait_ . krate (db)) , None) ; for super_trait in elaborate :: supertrait_def_ids (interner , trait_ . into ()) . skip (1) { if db . dyn_compatibility_of_trait (super_trait . 0) . is_some () { cb (DynCompatibilityViolation :: HasNonCompatibleSuperTrait (trait_)) ? ; } } dyn_compatibility_of_trait_with_callback (db , trait_ , cb) }
};
}
