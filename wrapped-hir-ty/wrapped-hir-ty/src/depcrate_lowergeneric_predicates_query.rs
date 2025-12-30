// Generated macro for generic_predicates_query (function)
macro_rules! Depcrate_lowergeneric_predicates_query {
() => {
// Module: crate::lower
// Provides: {"generic_predicates_query"}
// Dependencies: {}
# [doc = " Resolve the where clause(s) of an item with generics."] pub (crate) fn generic_predicates_query (db : & dyn HirDatabase , def : GenericDefId ,) -> GenericPredicates { generic_predicates_filtered_by (db , def , | _ , _ | true) . 0 }
};
}
