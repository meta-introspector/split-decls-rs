// Generated macro for ContainsNetLoose (type)
macro_rules! Depcrate_pg_expression_helper_typesContainsNetLoose {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"ContainsNetLoose"}
// Dependencies: {}
# [doc = " The return type of [`lhs.contains_or_eq(rhs)`](super::expression_methods::PgNetExpressionMethods::contains_or_eq)"] # [cfg (feature = "postgres_backend")] pub type ContainsNetLoose < Lhs , Rhs > = Grouped < super :: operators :: ContainsNetLoose < Lhs , AsExprOf < Rhs , Inet > > > ;
};
}
