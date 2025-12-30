// Generated macro for Index (type)
macro_rules! Depcrate_pg_expression_helper_typesIndex {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"Index"}
// Dependencies: {}
# [doc = " The return type of [`lhs.index(rhs)`](super::expression_methods::PgArrayExpressionMethods::index)"] # [cfg (feature = "postgres_backend")] pub type Index < Lhs , Rhs > = super :: operators :: ArrayIndex < Lhs , AsExprOf < Rhs , Integer > > ;
};
}
