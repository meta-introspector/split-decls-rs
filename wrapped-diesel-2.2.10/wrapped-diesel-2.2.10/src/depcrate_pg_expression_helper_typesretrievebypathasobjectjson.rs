// Generated macro for RetrieveByPathAsObjectJson (type)
macro_rules! Depcrate_pg_expression_helper_typesRetrieveByPathAsObjectJson {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"RetrieveByPathAsObjectJson"}
// Dependencies: {}
# [doc = " The return type of [`lhs.retrieve_by_path_as_object(rhs)`](super::expression_methods::PgAnyJsonExpressionMethods::retrieve_by_path_as_object)"] # [cfg (feature = "postgres_backend")] pub type RetrieveByPathAsObjectJson < Lhs , Rhs > = Grouped < super :: operators :: RetrieveByPathAsObjectJson < Lhs , AsExprOf < Rhs , Array < VarChar > > > > ;
};
}
