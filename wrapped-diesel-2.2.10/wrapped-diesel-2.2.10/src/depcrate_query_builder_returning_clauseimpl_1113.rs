// Generated macro for impl_1113 (impl)
macro_rules! Depcrate_query_builder_returning_clauseimpl_1113 {
() => {
// Module: crate::query_builder::returning_clause
// Provides: {"impl_1113"}
// Dependencies: {}
impl < S , T , U , V > ReturningClauseHelper < S > for UpdateStatement < T , U , V > where T : QuerySource , { type WithReturning = UpdateStatement < T , U , V , ReturningClause < S > > ; }
};
}
