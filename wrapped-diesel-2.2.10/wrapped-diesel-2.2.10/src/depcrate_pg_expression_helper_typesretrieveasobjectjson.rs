// Generated macro for RetrieveAsObjectJson (type)
macro_rules! Depcrate_pg_expression_helper_typesRetrieveAsObjectJson {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"RetrieveAsObjectJson"}
// Dependencies: {}
# [doc = " The return type of [`lhs.retrieve_as_object(rhs)`](super::expression_methods::PgAnyJsonExpressionMethods::retrieve_as_object)"] # [cfg (feature = "postgres_backend")] pub type RetrieveAsObjectJson < Lhs , Rhs , ST > = Grouped < super :: operators :: RetrieveAsObjectJson < Lhs , AsExprOf < Rhs , ST > > > ;
};
}
