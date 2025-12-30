// Generated macro for impl_867 (impl)
macro_rules! Depcrate_query_builder_delete_statementimpl_867 {
() => {
// Module: crate::query_builder::delete_statement
// Provides: {"impl_867"}
// Dependencies: {}
impl < T , U , Ret > Query for DeleteStatement < T , U , ReturningClause < Ret > > where T : Table , Ret : SelectableExpression < T > , { type SqlType = Ret :: SqlType ; }
};
}
