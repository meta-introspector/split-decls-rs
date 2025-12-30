// Generated macro for OverlapsWithNet (type)
macro_rules! Depcrate_pg_expression_helper_typesOverlapsWithNet {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"OverlapsWithNet"}
// Dependencies: {}
# [doc = " The return type of [`lhs.overlaps_with(rhs)`](super::expression_methods::PgNetExpressionMethods::overlaps_with)"] # [doc = " for network types"] # [cfg (feature = "postgres_backend")] pub type OverlapsWithNet < Lhs , Rhs > = Grouped < super :: operators :: OverlapsWith < Lhs , AsExprOf < Rhs , Inet > > > ;
};
}
