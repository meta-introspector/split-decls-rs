// Generated macro for impl_1003 (impl)
macro_rules! Depcrate_query_builder_insert_statement_insert_with_default_for_sqliteimpl_1003 {
() => {
// Module: crate::query_builder::insert_statement::insert_with_default_for_sqlite
// Provides: {"impl_1003"}
// Dependencies: {}
impl < V , T , QId , C , Op , const STATIC_QUERY_ID : bool > ExecuteDsl < C , Sqlite > for (Yes , InsertStatement < T , BatchInsert < Vec < ValuesClause < V , T > > , T , QId , STATIC_QUERY_ID > , Op > ,) where C : Connection < Backend = Sqlite > , T : Table + Copy + QueryId + 'static , T :: FromClause : QueryFragment < Sqlite > , Op : Copy + QueryId + QueryFragment < Sqlite > , V : InsertValues < Sqlite , T > + CanInsertInSingleQuery < Sqlite > + QueryId , { fn execute ((Yes , query) : Self , conn : & mut C) -> QueryResult < usize > { conn . transaction (| conn | { let mut result = 0 ; for record in & query . records . values { let stmt = InsertStatement :: new (query . target , record , query . operator , query . returning) ; result += stmt . execute (conn) ? ; } Ok (result) }) } }
};
}
