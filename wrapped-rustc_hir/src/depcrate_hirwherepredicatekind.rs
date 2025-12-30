// Generated macro for WherePredicateKind (enum)
macro_rules! Depcrate_hirWherePredicateKind {
() => {
// Module: crate::hir
// Provides: {"WherePredicateKind"}
// Dependencies: {}
# [doc = " The kind of a single predicate in a where-clause."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub enum WherePredicateKind < 'hir > { # [doc = " A type bound (e.g., `for<'c> Foo: Send + Clone + 'c`)."] BoundPredicate (WhereBoundPredicate < 'hir >) , # [doc = " A lifetime predicate (e.g., `'a: 'b + 'c`)."] RegionPredicate (WhereRegionPredicate < 'hir >) , # [doc = " An equality predicate (unsupported)."] EqPredicate (WhereEqPredicate < 'hir >) , }
};
}
