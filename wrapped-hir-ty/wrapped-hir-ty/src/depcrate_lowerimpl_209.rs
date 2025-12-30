// Generated macro for impl_209 (impl)
macro_rules! Depcrate_lowerimpl_209 {
() => {
// Module: crate::lower
// Provides: {"impl_209"}
// Dependencies: {}
# [salsa :: tracked] impl < 'db > GenericPredicates < 'db > { # [doc = " Resolve the where clause(s) of an item with generics."] # [doc = ""] # [doc = " Diagnostics are computed only for this item's predicates, not for parents."] # [salsa :: tracked (returns (ref) , unsafe (non_update_return_type))] pub fn query_with_diagnostics (db : & 'db dyn HirDatabase , def : GenericDefId ,) -> (GenericPredicates < 'db > , Diagnostics) { generic_predicates_filtered_by (db , def , PredicateFilter :: All , | _ | true) } }
};
}
