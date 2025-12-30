// Generated macro for impl_888 (impl)
macro_rules! Depcrate_query_builder_from_clauseimpl_888 {
() => {
// Module: crate::query_builder::from_clause
// Provides: {"impl_888"}
// Dependencies: {}
impl < DB > QueryFragment < DB , crate :: backend :: sql_dialect :: from_clause_syntax :: AnsiSqlFromClauseSyntax > for NoFromClause where DB : Backend < EmptyFromClauseSyntax = crate :: backend :: sql_dialect :: from_clause_syntax :: AnsiSqlFromClauseSyntax > , { fn walk_ast < 'b > (& 'b self , _pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { Ok (()) } }
};
}
