// Generated macro for impl_1425 (impl)
macro_rules! Depcrate_query_builder_upsert_on_conflict_clauseimpl_1425 {
() => {
// Module: crate::query_builder::upsert::on_conflict_clause
// Provides: {"impl_1425"}
// Dependencies: {}
impl < DB , Values , Target , Action , Expr > QueryFragment < DB > for OnConflictValues < Values , Target , Action , WhereClause < Expr > > where DB : Backend , DB :: OnConflictClause : sql_dialect :: on_conflict_clause :: SupportsOnConflictClause , DB :: OnConflictClause : sql_dialect :: on_conflict_clause :: SupportsOnConflictClauseWhere , Values : QueryFragment < DB > , Target : QueryFragment < DB > , Action : QueryFragment < DB > , WhereClause < Expr > : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { self . values . walk_ast (out . reborrow ()) ? ; out . push_sql (" ON CONFLICT") ; self . target . walk_ast (out . reborrow ()) ? ; self . action . walk_ast (out . reborrow ()) ? ; self . where_clause . walk_ast (out . reborrow ()) ? ; Ok (()) } }
};
}
