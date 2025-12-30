// Generated macro for impl_1046 (impl)
macro_rules! Depcrate_query_builder_insert_statementimpl_1046 {
() => {
// Module: crate::query_builder::insert_statement
// Provides: {"impl_1046"}
// Dependencies: {}
impl < DB > QueryFragment < DB , sql_dialect :: default_value_clause :: AnsiDefaultValueClause > for DefaultValues where DB : Backend + SqlDialect < DefaultValueClauseForInsert = sql_dialect :: default_value_clause :: AnsiDefaultValueClause , > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_sql ("DEFAULT VALUES") ; Ok (()) } }
};
}
