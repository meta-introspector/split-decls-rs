// Generated macro for RetrieveAsTextJson (type)
macro_rules! Depcrate_pg_expression_helper_typesRetrieveAsTextJson {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"RetrieveAsTextJson"}
// Dependencies: {}
# [doc = " The return type of [`lhs.retrieve_as_text(rhs)`](super::expression_methods::PgAnyJsonExpressionMethods::retrieve_as_text)"] # [cfg (feature = "postgres_backend")] pub type RetrieveAsTextJson < Lhs , Rhs , ST > = Grouped < super :: operators :: RetrieveAsTextJson < Lhs , AsExprOf < Rhs , ST > > > ;
};
}
