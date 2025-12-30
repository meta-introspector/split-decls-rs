// Generated macro for impl_939 (impl)
macro_rules! Depcrate_query_builder_insert_statement_batch_insertimpl_939 {
() => {
// Module: crate::query_builder::insert_statement::batch_insert
// Provides: {"impl_939"}
// Dependencies: {}
impl < T , DB > CanInsertInSingleQuery < DB > for Vec < T > where DB : Backend + SqlDialect < InsertWithDefaultKeyword = sql_dialect :: default_keyword_for_insert :: IsoSqlDefaultKeyword > , { fn rows_to_insert (& self) -> Option < usize > { Some (self . len ()) } }
};
}
