// Generated macro for impl_1007 (impl)
macro_rules! Depcrate_query_builder_insert_statement_insert_with_default_for_sqliteimpl_1007 {
() => {
// Module: crate::query_builder::insert_statement::insert_with_default_for_sqlite
// Provides: {"impl_1007"}
// Dependencies: {}
impl < V , T , QId , const STATIC_QUERY_ID : bool > CanInsertInSingleQuery < Sqlite > for SqliteBatchInsertWrapper < Vec < ValuesClause < V , T > > , T , QId , STATIC_QUERY_ID > where SqliteCanInsertInSingleQueryHelper < V > : CanInsertInSingleQuery < Sqlite > , { fn rows_to_insert (& self) -> Option < usize > { Some (self . 0 . values . len ()) } }
};
}
