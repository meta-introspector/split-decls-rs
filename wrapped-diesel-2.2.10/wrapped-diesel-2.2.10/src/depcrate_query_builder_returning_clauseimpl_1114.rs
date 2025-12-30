// Generated macro for impl_1114 (impl)
macro_rules! Depcrate_query_builder_returning_clauseimpl_1114 {
() => {
// Module: crate::query_builder::returning_clause
// Provides: {"impl_1114"}
// Dependencies: {}
impl < S , T , U > ReturningClauseHelper < S > for DeleteStatement < T , U > where T : QuerySource , { type WithReturning = DeleteStatement < T , U , ReturningClause < S > > ; }
};
}
