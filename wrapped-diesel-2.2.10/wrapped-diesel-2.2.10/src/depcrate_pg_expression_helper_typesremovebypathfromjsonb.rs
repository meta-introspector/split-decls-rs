// Generated macro for RemoveByPathFromJsonb (type)
macro_rules! Depcrate_pg_expression_helper_typesRemoveByPathFromJsonb {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"RemoveByPathFromJsonb"}
// Dependencies: {}
# [doc = " The return type of [`lhs.remove_by_path(rhs)`](super::expression_methods::PgJsonbExpressionMethods::remove_by_path)"] # [cfg (feature = "postgres_backend")] pub type RemoveByPathFromJsonb < Lhs , Rhs > = Grouped < super :: operators :: RemoveByPathFromJsonb < Lhs , AsExprOf < Rhs , Array < VarChar > > > > ;
};
}
