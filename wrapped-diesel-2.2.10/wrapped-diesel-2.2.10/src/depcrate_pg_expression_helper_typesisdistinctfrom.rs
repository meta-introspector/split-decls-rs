// Generated macro for IsDistinctFrom (type)
macro_rules! Depcrate_pg_expression_helper_typesIsDistinctFrom {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"IsDistinctFrom"}
// Dependencies: {}
# [doc = " The return type of [`lhs.is_distinct_from(rhs)`](super::expression_methods::PgExpressionMethods::is_distinct_from)"] # [cfg (feature = "postgres_backend")] pub type IsDistinctFrom < Lhs , Rhs > = Grouped < super :: operators :: IsDistinctFrom < Lhs , AsExpr < Rhs , Lhs > > > ;
};
}
