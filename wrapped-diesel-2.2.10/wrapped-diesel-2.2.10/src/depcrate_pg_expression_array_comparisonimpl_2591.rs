// Generated macro for impl_2591 (impl)
macro_rules! Depcrate_pg_expression_array_comparisonimpl_2591 {
() => {
// Module: crate::pg::expression::array_comparison
// Provides: {"impl_2591"}
// Dependencies: {}
impl < Expr , ST > Expression for Any < Expr > where Expr : Expression < SqlType = Array < ST > > , ST : SqlType + TypedExpressionType , { type SqlType = ST ; }
};
}
