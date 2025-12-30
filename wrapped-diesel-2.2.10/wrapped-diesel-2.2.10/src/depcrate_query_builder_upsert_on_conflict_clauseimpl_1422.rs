// Generated macro for impl_1422 (impl)
macro_rules! Depcrate_query_builder_upsert_on_conflict_clauseimpl_1422 {
() => {
// Module: crate::query_builder::upsert::on_conflict_clause
// Provides: {"impl_1422"}
// Dependencies: {}
impl < DB , Values , Target , Action , WhereClause > CanInsertInSingleQuery < DB > for OnConflictValues < Values , Target , Action , WhereClause > where DB : Backend , DB :: OnConflictClause : sql_dialect :: on_conflict_clause :: SupportsOnConflictClause , Values : CanInsertInSingleQuery < DB > , { fn rows_to_insert (& self) -> Option < usize > { self . values . rows_to_insert () } }
};
}
