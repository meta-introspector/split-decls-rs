// Generated macro for ImplTraitBound (struct)
macro_rules! Depcrate_implied_bounds_in_implsImplTraitBound {
() => {
// Module: crate::implied_bounds_in_impls
// Provides: {"ImplTraitBound"}
// Dependencies: {}
struct ImplTraitBound < 'tcx > { # [doc = " The span of the bound in the `impl Trait` type"] span : Span , # [doc = " The predicates defined in the trait referenced by this bound. This also contains the actual"] # [doc = " supertrait bounds"] predicates : & 'tcx [(ty :: Clause < 'tcx > , Span)] , # [doc = " The `DefId` of the trait being referenced by this bound"] trait_def_id : DefId , # [doc = " The generic arguments on the `impl Trait` bound"] args : & 'tcx [GenericArg < 'tcx >] , # [doc = " The associated item constraints of this bound"] constraints : & 'tcx [AssocItemConstraint < 'tcx >] , }
};
}
