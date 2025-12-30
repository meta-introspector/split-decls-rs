// Generated macro for AndNet (type)
macro_rules! Depcrate_pg_expression_helper_typesAndNet {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"AndNet"}
// Dependencies: {}
# [doc = " The return type of [`lsh.and(rhs)`](super::expression_methods::PgNetExpressionMethods::and) for network types"] # [cfg (feature = "postgres_backend")] pub type AndNet < Lhs , Rhs > = Grouped < super :: operators :: AndNet < Lhs , AsExprOf < Rhs , Inet > > > ;
};
}
