// Generated macro for RemoveFromJsonb (type)
macro_rules! Depcrate_pg_expression_helper_typesRemoveFromJsonb {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"RemoveFromJsonb"}
// Dependencies: {}
# [doc = " The return type of [`lhs.remove(rhs)`](super::expression_methods::PgJsonbExpressionMethods::remove)"] # [cfg (feature = "postgres_backend")] pub type RemoveFromJsonb < Lhs , Rhs , ST > = Grouped < super :: operators :: RemoveFromJsonb < Lhs , AsExprOf < Rhs , ST > > > ;
};
}
