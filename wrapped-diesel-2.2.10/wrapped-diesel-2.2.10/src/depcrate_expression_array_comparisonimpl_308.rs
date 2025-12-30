// Generated macro for impl_308 (impl)
macro_rules! Depcrate_expression_array_comparisonimpl_308 {
() => {
// Module: crate::expression::array_comparison
// Provides: {"impl_308"}
// Dependencies: {}
impl < ST , I , DB > QueryFragment < DB , sql_dialect :: array_comparison :: AnsiSqlArrayComparison > for Many < ST , I > where DB : Backend + HasSqlType < ST > + SqlDialect < ArrayComparison = sql_dialect :: array_comparison :: AnsiSqlArrayComparison > , ST : SingleValue , I : ToSql < ST , DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . unsafe_to_cache_prepared () ; let mut first = true ; for value in & self . values { if first { first = false ; } else { out . push_sql (", ") ; } out . push_bind_param (value) ? ; } Ok (()) } }
};
}
