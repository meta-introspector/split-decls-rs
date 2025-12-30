// Generated macro for impl_1026 (impl)
macro_rules! Depcrate_query_builder_insert_statementimpl_1026 {
() => {
// Module: crate::query_builder::insert_statement
// Provides: {"impl_1026"}
// Dependencies: {}
impl < T , U , Op , Ret > Query for InsertStatement < T , U , Op , ReturningClause < Ret > > where T : QuerySource , Ret : Expression + SelectableExpression < T > + NonAggregate , { type SqlType = Ret :: SqlType ; }
};
}
