// Generated macro for impl_1400 (impl)
macro_rules! Depcrate_query_builder_upsert_on_conflict_actionsimpl_1400 {
() => {
// Module: crate::query_builder::upsert::on_conflict_actions
// Provides: {"impl_1400"}
// Dependencies: {}
impl < DB , T , SD > QueryFragment < DB , SD > for DoNothing < T > where DB : Backend < OnConflictClause = SD > , SD : on_conflict_clause :: PgLikeOnConflictClause , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_sql (" DO NOTHING") ; Ok (()) } }
};
}
