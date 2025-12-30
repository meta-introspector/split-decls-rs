// Generated macro for impl_1408 (impl)
macro_rules! Depcrate_query_builder_upsert_on_conflict_actionsimpl_1408 {
() => {
// Module: crate::query_builder::upsert::on_conflict_actions
// Provides: {"impl_1408"}
// Dependencies: {}
impl < DB , T , SD > QueryFragment < DB , SD > for Excluded < T > where DB : Backend < OnConflictClause = SD > , T : Column , SD : on_conflict_clause :: PgLikeOnConflictClause , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_sql ("excluded.") ; out . push_identifier (T :: NAME) ? ; Ok (()) } }
};
}
