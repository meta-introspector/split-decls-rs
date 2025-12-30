// Generated macro for impl_508 (impl)
macro_rules! Depcrate_utilsimpl_508 {
() => {
// Module: crate::utils
// Provides: {"impl_508"}
// Dependencies: {}
impl MaybeIdent for syn :: WherePredicate { fn maybe_ident (& self) -> Option < & Ident > { match self { WherePredicate :: Type (syn :: PredicateType { bounded_ty : t , .. }) => { first_type_path_segment_ident (t) } WherePredicate :: Lifetime (syn :: PredicateLifetime { lifetime , .. }) => { Some (& lifetime . ident) } _ => None , } } }
};
}
