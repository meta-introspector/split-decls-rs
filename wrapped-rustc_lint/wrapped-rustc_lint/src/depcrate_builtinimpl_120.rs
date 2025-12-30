// Generated macro for impl_120 (impl)
macro_rules! Depcrate_builtinimpl_120 {
() => {
// Module: crate::builtin
// Provides: {"impl_120"}
// Dependencies: {}
impl TypeAliasBounds { pub (crate) fn affects_object_lifetime_defaults (pred : & hir :: WherePredicate < '_ >) -> bool { if let hir :: WherePredicateKind :: BoundPredicate (pred) = pred . kind && pred . bounds . iter () . any (| bound | matches ! (bound , hir :: GenericBound :: Outlives (_))) && pred . bound_generic_params . is_empty () && pred . bounded_ty . as_generic_param () . is_some () { return true ; } false } }
};
}
