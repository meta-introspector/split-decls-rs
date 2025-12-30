// Generated macro for impl_2214 (impl)
macro_rules! Depcrate_mysql_backendimpl_2214 {
() => {
// Module: crate::mysql::backend
// Provides: {"impl_2214"}
// Dependencies: {}
impl SqlDialect for Mysql { type ReturningClause = sql_dialect :: returning_clause :: DoesNotSupportReturningClause ; type OnConflictClause = MysqlOnConflictClause ; type InsertWithDefaultKeyword = sql_dialect :: default_keyword_for_insert :: IsoSqlDefaultKeyword ; type BatchInsertSupport = sql_dialect :: batch_insert_support :: PostgresLikeBatchInsertSupport ; type DefaultValueClauseForInsert = MysqlStyleDefaultValueClause ; type EmptyFromClauseSyntax = sql_dialect :: from_clause_syntax :: AnsiSqlFromClauseSyntax ; type SelectStatementSyntax = sql_dialect :: select_statement_syntax :: AnsiSqlSelectStatement ; type ExistsSyntax = sql_dialect :: exists_syntax :: AnsiSqlExistsSyntax ; type ArrayComparison = sql_dialect :: array_comparison :: AnsiSqlArrayComparison ; type ConcatClause = MysqlConcatClause ; type AliasSyntax = sql_dialect :: alias_syntax :: AsAliasSyntax ; }
};
}
