// Generated macro for impl_1125 (impl)
macro_rules! Depcrate_query_builder_select_clauseimpl_1125 {
() => {
// Module: crate::query_builder::select_clause
// Provides: {"impl_1125"}
// Dependencies: {}
impl < QS > QueryId for DefaultSelectClause < QS > where QS : AsQuerySource , < QS :: QuerySource as QuerySource > :: DefaultSelection : QueryId , { type QueryId = < < QS :: QuerySource as QuerySource > :: DefaultSelection as QueryId > :: QueryId ; const HAS_STATIC_QUERY_ID : bool = < < QS :: QuerySource as QuerySource > :: DefaultSelection as QueryId > :: HAS_STATIC_QUERY_ID ; }
};
}
