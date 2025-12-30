// Generated macro for IsContainedByNetLoose (type)
macro_rules! Depcrate_pg_expression_helper_typesIsContainedByNetLoose {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"IsContainedByNetLoose"}
// Dependencies: {}
# [doc = " The return type of [`lhs.is_contained_by_or_eq(rhs)`](super::expression_methods::PgNetExpressionMethods::is_contained_by_or_eq)"] # [cfg (feature = "postgres_backend")] pub type IsContainedByNetLoose < Lhs , Rhs > = Grouped < super :: operators :: IsContainedByNetLoose < Lhs , AsExprOf < Rhs , Inet > > > ;
};
}
