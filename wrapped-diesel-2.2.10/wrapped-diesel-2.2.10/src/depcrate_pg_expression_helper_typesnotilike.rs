// Generated macro for NotILike (type)
macro_rules! Depcrate_pg_expression_helper_typesNotILike {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"NotILike"}
// Dependencies: {}
# [doc = " The return type of [`lhs.not_ilike(rhs)`](super::expression_methods::PgTextExpressionMethods::not_ilike)"] # [cfg (feature = "postgres_backend")] pub type NotILike < Lhs , Rhs > = Grouped < super :: operators :: NotILike < Lhs , AsExprOf < Rhs , VarChar > > > ;
};
}
