// Generated macro for impl_1285 (impl)
macro_rules! Depcrate_query_builder_select_statementimpl_1285 {
() => {
// Module: crate::query_builder::select_statement
// Provides: {"impl_1285"}
// Dependencies: {}
impl < F , S , D , W , O , LOf , G , H , LC , DB > QueryFragment < DB , sql_dialect :: select_statement_syntax :: AnsiSqlSelectStatement > for SelectStatement < F , S , D , W , O , LOf , G , H , LC > where DB : Backend < SelectStatementSyntax = sql_dialect :: select_statement_syntax :: AnsiSqlSelectStatement , > , S : QueryFragment < DB > , F : QueryFragment < DB > , D : QueryFragment < DB > , W : QueryFragment < DB > , O : QueryFragment < DB > , LOf : QueryFragment < DB > , G : QueryFragment < DB > , H : QueryFragment < DB > , LC : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_sql ("SELECT ") ; self . distinct . walk_ast (out . reborrow ()) ? ; self . select . walk_ast (out . reborrow ()) ? ; self . from . walk_ast (out . reborrow ()) ? ; self . where_clause . walk_ast (out . reborrow ()) ? ; self . group_by . walk_ast (out . reborrow ()) ? ; self . having . walk_ast (out . reborrow ()) ? ; self . order . walk_ast (out . reborrow ()) ? ; self . limit_offset . walk_ast (out . reborrow ()) ? ; self . locking . walk_ast (out . reborrow ()) ? ; Ok (()) } }
};
}
