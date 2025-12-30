// Generated macro for impl_2892 (impl)
macro_rules! Depcrate_implied_bounds_in_implsimpl_2892 {
() => {
// Module: crate::implied_bounds_in_impls
// Provides: {"impl_2892"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for ImpliedBoundsInImpls { fn check_generics (& mut self , cx : & LateContext < 'tcx > , generics : & rustc_hir :: Generics < 'tcx >) { for predicate in generics . predicates { if let WherePredicateKind :: BoundPredicate (predicate) = predicate . kind && let PredicateOrigin :: ImplTrait = predicate . origin { check (cx , predicate . bounds) ; } } } fn check_ty (& mut self , cx : & LateContext < 'tcx > , ty : & rustc_hir :: Ty < 'tcx , AmbigArg >) { if let TyKind :: OpaqueDef (opaque_ty , ..) = ty . kind { check (cx , opaque_ty . bounds) ; } } }
};
}
