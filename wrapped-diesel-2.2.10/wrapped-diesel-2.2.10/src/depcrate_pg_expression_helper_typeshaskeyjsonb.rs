// Generated macro for HasKeyJsonb (type)
macro_rules! Depcrate_pg_expression_helper_typesHasKeyJsonb {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"HasKeyJsonb"}
// Dependencies: {}
# [doc = " The return type of [`lsh.has_key(rhs)`](super::expression_methods::PgJsonbExpressionMethods::has_key)"] # [cfg (feature = "postgres_backend")] pub type HasKeyJsonb < Lhs , Rhs > = Grouped < super :: operators :: HasKeyJsonb < Lhs , AsExprOf < Rhs , VarChar > > > ;
};
}
