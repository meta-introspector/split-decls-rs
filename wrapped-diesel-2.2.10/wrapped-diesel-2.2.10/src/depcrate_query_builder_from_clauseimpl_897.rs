// Generated macro for impl_897 (impl)
macro_rules! Depcrate_query_builder_from_clauseimpl_897 {
() => {
// Module: crate::query_builder::from_clause
// Provides: {"impl_897"}
// Dependencies: {}
impl < F : QuerySource > FromClause < F > { pub (crate) fn new (qs : F) -> Self { Self { from_clause : qs . from_clause () , source : qs , } } }
};
}
