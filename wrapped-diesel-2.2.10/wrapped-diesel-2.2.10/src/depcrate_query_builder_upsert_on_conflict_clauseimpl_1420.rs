// Generated macro for impl_1420 (impl)
macro_rules! Depcrate_query_builder_upsert_on_conflict_clauseimpl_1420 {
() => {
// Module: crate::query_builder::upsert::on_conflict_clause
// Provides: {"impl_1420"}
// Dependencies: {}
impl < Values , T > OnConflictValues < Values , NoConflictTarget , DoNothing < T > , NoWhereClause > { pub (crate) fn do_nothing (values : Values) -> Self { Self :: new (values , NoConflictTarget , DoNothing :: new () , NoWhereClause) } }
};
}
