// Generated macro for IsContainedByNet (type)
macro_rules! Depcrate_pg_expression_helper_typesIsContainedByNet {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"IsContainedByNet"}
// Dependencies: {}
# [doc = " The return type of [`lhs.is_contained_by(rhs)`]((super::expression_methods::PgNetExpressionMethods::is_contained_by)"] # [doc = " for network types"] # [cfg (feature = "postgres_backend")] pub type IsContainedByNet < Lhs , Rhs > = Grouped < super :: operators :: IsContainedByNet < Lhs , AsExprOf < Rhs , Inet > > > ;
};
}
