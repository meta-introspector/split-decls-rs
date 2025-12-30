// Generated macro for Or (type)
macro_rules! Depcrate_expression_helper_typesOr {
() => {
// Module: crate::expression::helper_types
// Provides: {"Or"}
// Dependencies: {}
# [doc = " The return type of"] # [doc = " [`lhs.or(rhs)`](crate::expression_methods::BoolExpressionMethods::or())"] pub type Or < Lhs , Rhs , ST = < Rhs as PreferredBoolSqlType > :: PreferredSqlType > = Grouped < super :: operators :: Or < Lhs , AsExprOf < Rhs , ST > > > ;
};
}
