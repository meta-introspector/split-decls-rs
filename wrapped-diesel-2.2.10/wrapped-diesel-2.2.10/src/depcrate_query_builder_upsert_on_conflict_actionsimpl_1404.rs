// Generated macro for impl_1404 (impl)
macro_rules! Depcrate_query_builder_upsert_on_conflict_actionsimpl_1404 {
() => {
// Module: crate::query_builder::upsert::on_conflict_actions
// Provides: {"impl_1404"}
// Dependencies: {}
impl < DB , T , Tab , SD > QueryFragment < DB , SD > for DoUpdate < T , Tab > where DB : Backend < OnConflictClause = SD > , T : QueryFragment < DB > , SD : on_conflict_clause :: PgLikeOnConflictClause , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . unsafe_to_cache_prepared () ; if self . changeset . is_noop (out . backend ()) ? { out . push_sql (" DO NOTHING") ; } else { out . push_sql (" DO UPDATE SET ") ; self . changeset . walk_ast (out . reborrow ()) ? ; } Ok (()) } }
};
}
