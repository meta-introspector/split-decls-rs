// Generated macro for generic_predicates_without_parent_query (function)
macro_rules! Depcrate_lowergeneric_predicates_without_parent_query {
() => {
// Module: crate::lower
// Provides: {"generic_predicates_without_parent_query"}
// Dependencies: {}
pub (crate) fn generic_predicates_without_parent_query (db : & dyn HirDatabase , def : GenericDefId ,) -> GenericPredicates { db . generic_predicates_without_parent_with_diagnostics (def) . 0 }
};
}
