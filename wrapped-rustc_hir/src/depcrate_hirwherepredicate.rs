// Generated macro for WherePredicate (struct)
macro_rules! Depcrate_hirWherePredicate {
() => {
// Module: crate::hir
// Provides: {"WherePredicate"}
// Dependencies: {}
# [doc = " A single predicate in a where-clause."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct WherePredicate < 'hir > { # [stable_hasher (ignore)] pub hir_id : HirId , pub span : Span , pub kind : & 'hir WherePredicateKind < 'hir > , }
};
}
