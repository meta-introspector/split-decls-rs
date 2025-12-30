// Generated macro for HasAnyKeyJsonb (type)
macro_rules! Depcrate_pg_expression_helper_typesHasAnyKeyJsonb {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"HasAnyKeyJsonb"}
// Dependencies: {}
# [doc = " The return type of [`lsh.has_any_key(rhs)`](super::expression_methods::PgJsonbExpressionMethods::has_any_key)"] # [cfg (feature = "postgres_backend")] pub type HasAnyKeyJsonb < Lhs , Rhs > = Grouped < super :: operators :: HasAnyKeyJsonb < Lhs , AsExprOf < Rhs , Array < VarChar > > > > ;
};
}
