// Generated macro for RetrieveByPathAsTextJson (type)
macro_rules! Depcrate_pg_expression_helper_typesRetrieveByPathAsTextJson {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"RetrieveByPathAsTextJson"}
// Dependencies: {}
# [doc = " The return type of [`lhs.retrieve_by_path_as_text(rhs)`](super::expression_methods::PgAnyJsonExpressionMethods::retrieve_by_path_as_text)"] # [cfg (feature = "postgres_backend")] pub type RetrieveByPathAsTextJson < Lhs , Rhs > = Grouped < super :: operators :: RetrieveByPathAsTextJson < Lhs , AsExprOf < Rhs , Array < VarChar > > > > ;
};
}
