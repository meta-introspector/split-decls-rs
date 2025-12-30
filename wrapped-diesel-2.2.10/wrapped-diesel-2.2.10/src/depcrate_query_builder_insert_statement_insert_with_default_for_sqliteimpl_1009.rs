// Generated macro for impl_1009 (impl)
macro_rules! Depcrate_query_builder_insert_statement_insert_with_default_for_sqliteimpl_1009 {
() => {
// Module: crate::query_builder::insert_statement::insert_with_default_for_sqlite
// Provides: {"impl_1009"}
// Dependencies: {}
impl < V , T , QId , const STATIC_QUERY_ID : bool > QueryId for SqliteBatchInsertWrapper < V , T , QId , STATIC_QUERY_ID > where BatchInsert < V , T , QId , STATIC_QUERY_ID > : QueryId , { type QueryId = < BatchInsert < V , T , QId , STATIC_QUERY_ID > as QueryId > :: QueryId ; const HAS_STATIC_QUERY_ID : bool = < BatchInsert < V , T , QId , STATIC_QUERY_ID > as QueryId > :: HAS_STATIC_QUERY_ID ; }
};
}
