// Generated macro for impl_1112 (impl)
macro_rules! Depcrate_query_builder_returning_clauseimpl_1112 {
() => {
// Module: crate::query_builder::returning_clause
// Provides: {"impl_1112"}
// Dependencies: {}
impl < S , T , U , Op > ReturningClauseHelper < S > for InsertStatement < T , U , Op > where T : QuerySource , { type WithReturning = InsertStatement < T , U , Op , ReturningClause < S > > ; }
};
}
