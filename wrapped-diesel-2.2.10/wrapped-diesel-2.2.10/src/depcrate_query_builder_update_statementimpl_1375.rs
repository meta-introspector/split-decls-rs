// Generated macro for impl_1375 (impl)
macro_rules! Depcrate_query_builder_update_statementimpl_1375 {
() => {
// Module: crate::query_builder::update_statement
// Provides: {"impl_1375"}
// Dependencies: {}
impl < T , U , V , Ret > Query for UpdateStatement < T , U , V , ReturningClause < Ret > > where T : Table , Ret : Expression + SelectableExpression < T > + ValidGrouping < () > , Ret :: IsAggregate : MixedAggregates < is_aggregate :: No , Output = is_aggregate :: No > , { type SqlType = Ret :: SqlType ; }
};
}
