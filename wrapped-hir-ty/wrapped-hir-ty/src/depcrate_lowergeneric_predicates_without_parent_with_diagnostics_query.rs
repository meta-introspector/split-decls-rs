// Generated macro for generic_predicates_without_parent_with_diagnostics_query (function)
macro_rules! Depcrate_lowergeneric_predicates_without_parent_with_diagnostics_query {
() => {
// Module: crate::lower
// Provides: {"generic_predicates_without_parent_with_diagnostics_query"}
// Dependencies: {}
# [doc = " Resolve the where clause(s) of an item with generics,"] # [doc = " except the ones inherited from the parent"] pub (crate) fn generic_predicates_without_parent_with_diagnostics_query (db : & dyn HirDatabase , def : GenericDefId ,) -> (GenericPredicates , Diagnostics) { generic_predicates_filtered_by (db , def , | _ , d | d == def) }
};
}
