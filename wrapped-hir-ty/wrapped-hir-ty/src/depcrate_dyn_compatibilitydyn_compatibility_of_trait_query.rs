// Generated macro for dyn_compatibility_of_trait_query (function)
macro_rules! Depcrate_dyn_compatibilitydyn_compatibility_of_trait_query {
() => {
// Module: crate::dyn_compatibility
// Provides: {"dyn_compatibility_of_trait_query"}
// Dependencies: {}
pub fn dyn_compatibility_of_trait_query (db : & dyn HirDatabase , trait_ : TraitId ,) -> Option < DynCompatibilityViolation > { let mut res = None ; _ = dyn_compatibility_of_trait_with_callback (db , trait_ , & mut | osv | { res = Some (osv) ; ControlFlow :: Break (()) }) ; res }
};
}
