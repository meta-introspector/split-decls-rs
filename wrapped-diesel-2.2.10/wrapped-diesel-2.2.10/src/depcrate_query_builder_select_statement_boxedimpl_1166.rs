// Generated macro for impl_1166 (impl)
macro_rules! Depcrate_query_builder_select_statement_boxedimpl_1166 {
() => {
// Module: crate::query_builder::select_statement::boxed
// Provides: {"impl_1166"}
// Dependencies: {}
impl < 'a , ST , QS , DB , GB > QueryFragment < DB , sql_dialect :: select_statement_syntax :: AnsiSqlSelectStatement > for BoxedSelectStatement < 'a , ST , QS , DB , GB > where DB : Backend < SelectStatementSyntax = sql_dialect :: select_statement_syntax :: AnsiSqlSelectStatement , > + DieselReserveSpecialization , QS : QueryFragment < DB > , BoxedLimitOffsetClause < 'a , DB > : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { self . build_query (out , | where_clause , out | where_clause . walk_ast (out)) } }
};
}
