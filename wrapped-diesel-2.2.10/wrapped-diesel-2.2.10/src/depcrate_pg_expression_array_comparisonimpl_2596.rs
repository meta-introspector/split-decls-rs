// Generated macro for impl_2596 (impl)
macro_rules! Depcrate_pg_expression_array_comparisonimpl_2596 {
() => {
// Module: crate::pg::expression::array_comparison
// Provides: {"impl_2596"}
// Dependencies: {}
impl < Expr , ST > Expression for All < Expr > where Expr : Expression < SqlType = Array < ST > > , ST : SqlType + TypedExpressionType , { type SqlType = ST ; }
};
}
