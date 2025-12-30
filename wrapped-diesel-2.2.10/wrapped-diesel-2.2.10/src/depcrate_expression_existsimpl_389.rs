// Generated macro for impl_389 (impl)
macro_rules! Depcrate_expression_existsimpl_389 {
() => {
// Module: crate::expression::exists
// Provides: {"impl_389"}
// Dependencies: {}
impl < T , DB > QueryFragment < DB , sql_dialect :: exists_syntax :: AnsiSqlExistsSyntax > for Exists < T > where DB : Backend + SqlDialect < ExistsSyntax = sql_dialect :: exists_syntax :: AnsiSqlExistsSyntax > , T : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_sql ("EXISTS (") ; self . subselect . walk_ast (out . reborrow ()) ? ; out . push_sql (")") ; Ok (()) } }
};
}
