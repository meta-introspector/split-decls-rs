// Generated macro for impl_935 (impl)
macro_rules! Depcrate_query_builder_insert_statement_batch_insertimpl_935 {
() => {
// Module: crate::query_builder::insert_statement::batch_insert
// Provides: {"impl_935"}
// Dependencies: {}
impl < T , Table , QId , DB , const HAS_STATIC_QUERY_ID : bool > CanInsertInSingleQuery < DB > for BatchInsert < T , Table , QId , HAS_STATIC_QUERY_ID > where T : CanInsertInSingleQuery < DB > , DB : Backend + SqlDialect < InsertWithDefaultKeyword = sql_dialect :: default_keyword_for_insert :: IsoSqlDefaultKeyword > { fn rows_to_insert (& self) -> Option < usize > { self . values . rows_to_insert () } }
};
}
