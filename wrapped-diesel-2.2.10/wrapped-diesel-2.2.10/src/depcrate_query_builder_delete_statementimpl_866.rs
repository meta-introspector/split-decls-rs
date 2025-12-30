// Generated macro for impl_866 (impl)
macro_rules! Depcrate_query_builder_delete_statementimpl_866 {
() => {
// Module: crate::query_builder::delete_statement
// Provides: {"impl_866"}
// Dependencies: {}
impl < T , U > AsQuery for DeleteStatement < T , U , NoReturningClause > where T : Table , T :: AllColumns : SelectableExpression < T > , DeleteStatement < T , U , ReturningClause < T :: AllColumns > > : Query , { type SqlType = < Self :: Query as Query > :: SqlType ; type Query = DeleteStatement < T , U , ReturningClause < T :: AllColumns > > ; fn as_query (self) -> Self :: Query { self . returning (T :: all_columns ()) } }
};
}
