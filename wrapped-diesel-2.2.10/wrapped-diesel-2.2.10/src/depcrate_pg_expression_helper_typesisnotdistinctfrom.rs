// Generated macro for IsNotDistinctFrom (type)
macro_rules! Depcrate_pg_expression_helper_typesIsNotDistinctFrom {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"IsNotDistinctFrom"}
// Dependencies: {}
# [doc = " The return type of [`lhs.is_not_distinct_from(rhs)`](super::expression_methods::PgExpressionMethods::is_not_distinct_from)"] # [cfg (feature = "postgres_backend")] pub type IsNotDistinctFrom < Lhs , Rhs > = Grouped < super :: operators :: IsNotDistinctFrom < Lhs , AsExpr < Rhs , Lhs > > > ;
};
}
