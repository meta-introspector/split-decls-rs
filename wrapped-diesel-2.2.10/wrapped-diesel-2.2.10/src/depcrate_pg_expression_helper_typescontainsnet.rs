// Generated macro for ContainsNet (type)
macro_rules! Depcrate_pg_expression_helper_typesContainsNet {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"ContainsNet"}
// Dependencies: {}
# [doc = " The return type of [`lhs.contains(rhs)`](super::expression_methods::PgNetExpressionMethods::contains)"] # [doc = " for network types"] # [cfg (feature = "postgres_backend")] pub type ContainsNet < Lhs , Rhs > = Grouped < super :: operators :: ContainsNet < Lhs , AsExprOf < Rhs , Inet > > > ;
};
}
