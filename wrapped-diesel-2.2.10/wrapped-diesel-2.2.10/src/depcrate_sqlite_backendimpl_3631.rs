// Generated macro for impl_3631 (impl)
macro_rules! Depcrate_sqlite_backendimpl_3631 {
() => {
// Module: crate::sqlite::backend
// Provides: {"impl_3631"}
// Dependencies: {}
impl SqlDialect for Sqlite { # [cfg (not (feature = "returning_clauses_for_sqlite_3_35"))] type ReturningClause = sql_dialect :: returning_clause :: DoesNotSupportReturningClause ; # [cfg (feature = "returning_clauses_for_sqlite_3_35")] type ReturningClause = SqliteReturningClause ; type OnConflictClause = SqliteOnConflictClause ; type InsertWithDefaultKeyword = sql_dialect :: default_keyword_for_insert :: DoesNotSupportDefaultKeyword ; type BatchInsertSupport = SqliteBatchInsert ; type ConcatClause = sql_dialect :: concat_clause :: ConcatWithPipesClause ; type DefaultValueClauseForInsert = sql_dialect :: default_value_clause :: AnsiDefaultValueClause ; type EmptyFromClauseSyntax = sql_dialect :: from_clause_syntax :: AnsiSqlFromClauseSyntax ; type SelectStatementSyntax = sql_dialect :: select_statement_syntax :: AnsiSqlSelectStatement ; type ExistsSyntax = sql_dialect :: exists_syntax :: AnsiSqlExistsSyntax ; type ArrayComparison = sql_dialect :: array_comparison :: AnsiSqlArrayComparison ; type AliasSyntax = sql_dialect :: alias_syntax :: AsAliasSyntax ; }
};
}
