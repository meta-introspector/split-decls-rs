// Generated macro for impl_1110 (impl)
macro_rules! Depcrate_query_builder_returning_clauseimpl_1110 {
() => {
// Module: crate::query_builder::returning_clause
// Provides: {"impl_1110"}
// Dependencies: {}
impl < Expr , DB > QueryFragment < DB , crate :: backend :: sql_dialect :: returning_clause :: PgLikeReturningClause > for ReturningClause < Expr > where DB : Backend < ReturningClause = crate :: backend :: sql_dialect :: returning_clause :: PgLikeReturningClause , > , Expr : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_sql (" RETURNING ") ; self . 0 . walk_ast (out . reborrow ()) ? ; Ok (()) } }
};
}
