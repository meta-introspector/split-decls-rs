// Generated macro for DifferenceNet (type)
macro_rules! Depcrate_pg_expression_helper_typesDifferenceNet {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"DifferenceNet"}
// Dependencies: {}
# [doc = " The return type of [`lsh.diff(rhs)`](super::expression_methods::PgNetExpressionMethods::diff)"] # [cfg (feature = "postgres_backend")] pub type DifferenceNet < Lhs , Rhs > = Grouped < super :: operators :: DifferenceNet < Lhs , AsExprOf < Rhs , Inet > > > ;
};
}
