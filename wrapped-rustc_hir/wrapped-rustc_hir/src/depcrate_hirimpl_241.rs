// Generated macro for impl_241 (impl)
macro_rules! Depcrate_hirimpl_241 {
() => {
// Module: crate::hir
// Provides: {"impl_241"}
// Dependencies: {}
impl < 'hir > WherePredicateKind < 'hir > { pub fn in_where_clause (& self) -> bool { match self { WherePredicateKind :: BoundPredicate (p) => p . origin == PredicateOrigin :: WhereClause , WherePredicateKind :: RegionPredicate (p) => p . in_where_clause , WherePredicateKind :: EqPredicate (_) => false , } } pub fn bounds (& self) -> GenericBounds < 'hir > { match self { WherePredicateKind :: BoundPredicate (p) => p . bounds , WherePredicateKind :: RegionPredicate (p) => p . bounds , WherePredicateKind :: EqPredicate (_) => & [] , } } }
};
}
