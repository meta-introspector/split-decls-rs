// Generated macro for impl_3171 (impl)
macro_rules! Depcrate_pg_backendimpl_3171 {
() => {
// Module: crate::pg::backend
// Provides: {"impl_3171"}
// Dependencies: {}
impl SqlDialect for Pg { type ReturningClause = sql_dialect :: returning_clause :: PgLikeReturningClause ; type OnConflictClause = PgOnConflictClause ; type InsertWithDefaultKeyword = sql_dialect :: default_keyword_for_insert :: IsoSqlDefaultKeyword ; type BatchInsertSupport = sql_dialect :: batch_insert_support :: PostgresLikeBatchInsertSupport ; type ConcatClause = sql_dialect :: concat_clause :: ConcatWithPipesClause ; type DefaultValueClauseForInsert = sql_dialect :: default_value_clause :: AnsiDefaultValueClause ; type EmptyFromClauseSyntax = sql_dialect :: from_clause_syntax :: AnsiSqlFromClauseSyntax ; type SelectStatementSyntax = sql_dialect :: select_statement_syntax :: AnsiSqlSelectStatement ; type ExistsSyntax = sql_dialect :: exists_syntax :: AnsiSqlExistsSyntax ; type ArrayComparison = PgStyleArrayComparison ; type AliasSyntax = sql_dialect :: alias_syntax :: AsAliasSyntax ; }
};
}
