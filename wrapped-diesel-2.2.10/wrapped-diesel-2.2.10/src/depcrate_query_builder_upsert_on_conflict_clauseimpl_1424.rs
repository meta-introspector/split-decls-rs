// Generated macro for impl_1424 (impl)
macro_rules! Depcrate_query_builder_upsert_on_conflict_clauseimpl_1424 {
() => {
// Module: crate::query_builder::upsert::on_conflict_clause
// Provides: {"impl_1424"}
// Dependencies: {}
impl < DB , Values , Target , Action , SD > QueryFragment < DB , SD > for OnConflictValues < Values , Target , Action , NoWhereClause > where DB : Backend < OnConflictClause = SD > , SD : sql_dialect :: on_conflict_clause :: PgLikeOnConflictClause , Values : QueryFragment < DB > , Target : QueryFragment < DB > , Action : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { self . values . walk_ast (out . reborrow ()) ? ; out . push_sql (" ON CONFLICT") ; self . target . walk_ast (out . reborrow ()) ? ; self . action . walk_ast (out . reborrow ()) ? ; Ok (()) } }
};
}
