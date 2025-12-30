// Generated macro for WhereRegionPredicate (struct)
macro_rules! Depcrate_hirWhereRegionPredicate {
() => {
// Module: crate::hir
// Provides: {"WhereRegionPredicate"}
// Dependencies: {}
# [doc = " A lifetime predicate (e.g., `'a: 'b + 'c`)."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct WhereRegionPredicate < 'hir > { pub in_where_clause : bool , pub lifetime : & 'hir Lifetime , pub bounds : GenericBounds < 'hir > , }
};
}
