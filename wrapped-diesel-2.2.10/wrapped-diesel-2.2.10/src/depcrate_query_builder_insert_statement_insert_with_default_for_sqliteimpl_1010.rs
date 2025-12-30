// Generated macro for impl_1010 (impl)
macro_rules! Depcrate_query_builder_insert_statement_insert_with_default_for_sqliteimpl_1010 {
() => {
// Module: crate::query_builder::insert_statement::insert_with_default_for_sqlite
// Provides: {"impl_1010"}
// Dependencies: {}
impl < V , T , QId , C , Op , const STATIC_QUERY_ID : bool > ExecuteDsl < C , Sqlite > for (No , InsertStatement < T , BatchInsert < V , T , QId , STATIC_QUERY_ID > , Op > ,) where C : Connection < Backend = Sqlite > , T : Table + QueryId + 'static , T :: FromClause : QueryFragment < Sqlite > , Op : QueryFragment < Sqlite > + QueryId , SqliteBatchInsertWrapper < V , T , QId , STATIC_QUERY_ID > : QueryFragment < Sqlite > + QueryId + CanInsertInSingleQuery < Sqlite > , { fn execute ((No , query) : Self , conn : & mut C) -> QueryResult < usize > { let query = InsertStatement { records : SqliteBatchInsertWrapper (query . records) , operator : query . operator , target : query . target , returning : query . returning , into_clause : query . into_clause , } ; query . execute (conn) } }
};
}
