// Generated macro for impl_893 (impl)
macro_rules! Depcrate_query_builder_from_clauseimpl_893 {
() => {
// Module: crate::query_builder::from_clause
// Provides: {"impl_893"}
// Dependencies: {}
impl < F > QueryId for FromClause < F > where F : QuerySource + QueryId , { type QueryId = F :: QueryId ; const HAS_STATIC_QUERY_ID : bool = F :: HAS_STATIC_QUERY_ID ; }
};
}
