// Generated macro for impl_1025 (impl)
macro_rules! Depcrate_query_builder_insert_statementimpl_1025 {
() => {
// Module: crate::query_builder::insert_statement
// Provides: {"impl_1025"}
// Dependencies: {}
impl < T , U , Op > AsQuery for InsertStatement < T , U , Op , NoReturningClause > where T : Table , InsertStatement < T , U , Op , ReturningClause < T :: AllColumns > > : Query , { type SqlType = < Self :: Query as Query > :: SqlType ; type Query = InsertStatement < T , U , Op , ReturningClause < T :: AllColumns > > ; fn as_query (self) -> Self :: Query { self . returning (T :: all_columns ()) } }
};
}
