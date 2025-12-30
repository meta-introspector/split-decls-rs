// Generated macro for impl_1433 (impl)
macro_rules! Depcrate_query_builder_upsert_on_conflict_targetimpl_1433 {
() => {
// Module: crate::query_builder::upsert::on_conflict_target
// Provides: {"impl_1433"}
// Dependencies: {}
impl < DB > QueryFragment < DB > for NoConflictTarget where DB : Backend , DB :: OnConflictClause : sql_dialect :: on_conflict_clause :: SupportsOnConflictClause , { fn walk_ast < 'b > (& 'b self , _ : AstPass < '_ , 'b , DB >) -> QueryResult < () > { Ok (()) } }
};
}
