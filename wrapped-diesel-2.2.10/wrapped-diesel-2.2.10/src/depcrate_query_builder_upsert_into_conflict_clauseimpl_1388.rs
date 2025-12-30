// Generated macro for impl_1388 (impl)
macro_rules! Depcrate_query_builder_upsert_into_conflict_clauseimpl_1388 {
() => {
// Module: crate::query_builder::upsert::into_conflict_clause
// Provides: {"impl_1388"}
// Dependencies: {}
impl < V , Tab , QId , const STATIC_QUERY_ID : bool > IntoConflictValueClause for BatchInsert < V , Tab , QId , STATIC_QUERY_ID > { type ValueClause = Self ; fn into_value_clause (self) -> Self :: ValueClause { self } }
};
}
