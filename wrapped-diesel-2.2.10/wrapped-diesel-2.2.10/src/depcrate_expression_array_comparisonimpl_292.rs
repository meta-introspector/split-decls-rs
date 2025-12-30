// Generated macro for impl_292 (impl)
macro_rules! Depcrate_expression_array_comparisonimpl_292 {
() => {
// Module: crate::expression::array_comparison
// Provides: {"impl_292"}
// Dependencies: {}
impl < T , U , DB > QueryFragment < DB , sql_dialect :: array_comparison :: AnsiSqlArrayComparison > for NotIn < T , U > where DB : Backend + SqlDialect < ArrayComparison = sql_dialect :: array_comparison :: AnsiSqlArrayComparison > , T : QueryFragment < DB > , U : QueryFragment < DB > + MaybeEmpty , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { if self . values . is_empty () { out . push_sql ("1=1") ; } else { self . left . walk_ast (out . reborrow ()) ? ; out . push_sql (" NOT IN (") ; self . values . walk_ast (out . reborrow ()) ? ; out . push_sql (")") ; } Ok (()) } }
};
}
