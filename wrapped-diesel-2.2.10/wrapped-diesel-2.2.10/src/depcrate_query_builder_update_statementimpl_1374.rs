// Generated macro for impl_1374 (impl)
macro_rules! Depcrate_query_builder_update_statementimpl_1374 {
() => {
// Module: crate::query_builder::update_statement
// Provides: {"impl_1374"}
// Dependencies: {}
impl < T , U , V > AsQuery for UpdateStatement < T , U , V , NoReturningClause > where T : Table , UpdateStatement < T , U , V , ReturningClause < T :: AllColumns > > : Query , T :: AllColumns : ValidGrouping < () > , < T :: AllColumns as ValidGrouping < () > > :: IsAggregate : MixedAggregates < is_aggregate :: No , Output = is_aggregate :: No > , { type SqlType = < Self :: Query as Query > :: SqlType ; type Query = UpdateStatement < T , U , V , ReturningClause < T :: AllColumns > > ; fn as_query (self) -> Self :: Query { self . returning (T :: all_columns ()) } }
};
}
