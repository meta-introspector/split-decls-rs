// Generated macro for NotSimilarTo (type)
macro_rules! Depcrate_pg_expression_helper_typesNotSimilarTo {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"NotSimilarTo"}
// Dependencies: {}
# [doc = " The return type of [`lhs.not_similar_to(rhs)`](super::expression_methods::PgTextExpressionMethods::not_similar_to)"] # [cfg (feature = "postgres_backend")] pub type NotSimilarTo < Lhs , Rhs > = Grouped < super :: operators :: NotSimilarTo < Lhs , AsExprOf < Rhs , VarChar > > > ;
};
}
