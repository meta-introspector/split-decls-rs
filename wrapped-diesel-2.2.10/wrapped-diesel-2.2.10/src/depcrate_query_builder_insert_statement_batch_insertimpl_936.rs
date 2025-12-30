// Generated macro for impl_936 (impl)
macro_rules! Depcrate_query_builder_insert_statement_batch_insertimpl_936 {
() => {
// Module: crate::query_builder::insert_statement::batch_insert
// Provides: {"impl_936"}
// Dependencies: {}
impl < T , DB , const N : usize > CanInsertInSingleQuery < DB > for [T ; N] where DB : Backend + SqlDialect < InsertWithDefaultKeyword = sql_dialect :: default_keyword_for_insert :: IsoSqlDefaultKeyword > { fn rows_to_insert (& self) -> Option < usize > { Some (N) } }
};
}
