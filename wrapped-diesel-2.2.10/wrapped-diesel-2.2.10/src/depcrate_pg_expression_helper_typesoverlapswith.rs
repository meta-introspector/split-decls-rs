// Generated macro for OverlapsWith (type)
macro_rules! Depcrate_pg_expression_helper_typesOverlapsWith {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"OverlapsWith"}
// Dependencies: {}
# [doc = " The return type of [`lhs.overlaps_with(rhs)`](super::expression_methods::PgArrayExpressionMethods::overlaps_with)"] # [cfg (feature = "postgres_backend")] pub type OverlapsWith < Lhs , Rhs > = Grouped < super :: operators :: OverlapsWith < Lhs , AsExpr < Rhs , Lhs > > > ;
};
}
