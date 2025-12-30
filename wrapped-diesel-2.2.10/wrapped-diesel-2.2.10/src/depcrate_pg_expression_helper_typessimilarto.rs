// Generated macro for SimilarTo (type)
macro_rules! Depcrate_pg_expression_helper_typesSimilarTo {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"SimilarTo"}
// Dependencies: {}
# [doc = " The return type of [`lhs.similar_to(rhs)`](super::expression_methods::PgTextExpressionMethods::similar_to)"] # [cfg (feature = "postgres_backend")] pub type SimilarTo < Lhs , Rhs > = Grouped < super :: operators :: SimilarTo < Lhs , AsExprOf < Rhs , VarChar > > > ;
};
}
