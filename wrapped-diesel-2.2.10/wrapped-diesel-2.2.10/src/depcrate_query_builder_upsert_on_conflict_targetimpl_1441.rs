// Generated macro for impl_1441 (impl)
macro_rules! Depcrate_query_builder_upsert_on_conflict_targetimpl_1441 {
() => {
// Module: crate::query_builder::upsert::on_conflict_target
// Provides: {"impl_1441"}
// Dependencies: {}
impl < DB , T , SP > QueryFragment < DB , SP > for ConflictTarget < (T ,) > where DB : Backend < OnConflictClause = SP > , SP : sql_dialect :: on_conflict_clause :: PgLikeOnConflictClause , T : Column , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_sql (" (") ; out . push_identifier (T :: NAME) ? ; out . push_sql (")") ; Ok (()) } }
};
}
