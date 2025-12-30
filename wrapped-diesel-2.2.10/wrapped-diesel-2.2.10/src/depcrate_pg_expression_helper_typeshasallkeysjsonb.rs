// Generated macro for HasAllKeysJsonb (type)
macro_rules! Depcrate_pg_expression_helper_typesHasAllKeysJsonb {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"HasAllKeysJsonb"}
// Dependencies: {}
# [doc = " The return type of [`lsh.has_all_keys(rhs)`](super::expression_methods::PgJsonbExpressionMethods::has_all_keys)"] # [cfg (feature = "postgres_backend")] pub type HasAllKeysJsonb < Lhs , Rhs > = Grouped < super :: operators :: HasAllKeysJsonb < Lhs , AsExprOf < Rhs , Array < VarChar > > > > ;
};
}
