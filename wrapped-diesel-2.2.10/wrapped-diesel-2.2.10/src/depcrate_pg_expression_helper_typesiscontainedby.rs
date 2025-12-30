// Generated macro for IsContainedBy (type)
macro_rules! Depcrate_pg_expression_helper_typesIsContainedBy {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"IsContainedBy"}
// Dependencies: {}
# [doc = " The return type of [`lhs.is_contained_by(rhs)`](super::expression_methods::PgArrayExpressionMethods::is_contained_by)"] # [cfg (feature = "postgres_backend")] pub type IsContainedBy < Lhs , Rhs > = Grouped < super :: operators :: IsContainedBy < Lhs , AsExpr < Rhs , Lhs > > > ;
};
}
