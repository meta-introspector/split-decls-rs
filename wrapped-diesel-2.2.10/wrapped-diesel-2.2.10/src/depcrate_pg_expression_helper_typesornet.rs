// Generated macro for OrNet (type)
macro_rules! Depcrate_pg_expression_helper_typesOrNet {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"OrNet"}
// Dependencies: {}
# [doc = " The return type of [`lsh.or(rhs)`](super::expression_methods::PgNetExpressionMethods::or) for network types"] # [cfg (feature = "postgres_backend")] pub type OrNet < Lhs , Rhs > = Grouped < super :: operators :: OrNet < Lhs , AsExprOf < Rhs , Inet > > > ;
};
}
