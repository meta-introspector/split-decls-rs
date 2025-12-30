// Generated macro for RegionCtxt (enum)
macro_rules! Depcrate_renumberRegionCtxt {
() => {
// Module: crate::renumber
// Provides: {"RegionCtxt"}
// Dependencies: {}
# [derive (Copy , Clone , Debug , Eq , PartialEq , Hash)] pub (crate) enum RegionCtxt { Location (Location) , TyContext (TyContext) , Free (Symbol) , LateBound (Symbol) , Existential (Option < Symbol >) , Placeholder (Symbol) , Unknown , }
};
}
