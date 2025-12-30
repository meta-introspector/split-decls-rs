// Generated macro for impl_290 (impl)
macro_rules! Depcrate_expression_array_comparisonimpl_290 {
() => {
// Module: crate::expression::array_comparison
// Provides: {"impl_290"}
// Dependencies: {}
impl < T , U , DB > QueryFragment < DB , sql_dialect :: array_comparison :: AnsiSqlArrayComparison > for In < T , U > where DB : Backend + SqlDialect < ArrayComparison = sql_dialect :: array_comparison :: AnsiSqlArrayComparison > , T : QueryFragment < DB > , U : QueryFragment < DB > + MaybeEmpty , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { if self . values . is_empty () { out . push_sql ("1=0") ; } else { self . left . walk_ast (out . reborrow ()) ? ; out . push_sql (" IN (") ; self . values . walk_ast (out . reborrow ()) ? ; out . push_sql (")") ; } Ok (()) } }
};
}
