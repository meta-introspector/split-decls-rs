// Generated macro for impl_1439 (impl)
macro_rules! Depcrate_query_builder_upsert_on_conflict_targetimpl_1439 {
() => {
// Module: crate::query_builder::upsert::on_conflict_target
// Provides: {"impl_1439"}
// Dependencies: {}
impl < DB , ST , SP > QueryFragment < DB , SP > for ConflictTarget < SqlLiteral < ST > > where DB : Backend < OnConflictClause = SP > , SP : sql_dialect :: on_conflict_clause :: PgLikeOnConflictClause , SqlLiteral < ST > : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_sql (" ") ; self . 0 . walk_ast (out . reborrow ()) ? ; Ok (()) } }
};
}
