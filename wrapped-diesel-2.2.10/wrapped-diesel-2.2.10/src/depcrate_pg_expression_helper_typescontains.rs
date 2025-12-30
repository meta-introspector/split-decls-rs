// Generated macro for Contains (type)
macro_rules! Depcrate_pg_expression_helper_typesContains {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"Contains"}
// Dependencies: {}
# [doc = " The return type of [`lhs.contains(rhs)`](super::expression_methods::PgArrayExpressionMethods::contains)"] # [doc = " for array expressions"] # [cfg (feature = "postgres_backend")] pub type Contains < Lhs , Rhs > = Grouped < super :: operators :: Contains < Lhs , AsExpr < Rhs , Lhs > > > ;
};
}
