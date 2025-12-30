// Generated macro for And (type)
macro_rules! Depcrate_expression_helper_typesAnd {
() => {
// Module: crate::expression::helper_types
// Provides: {"And"}
// Dependencies: {}
# [doc = " The return type of"] # [doc = " [`lhs.and(rhs)`](crate::expression_methods::BoolExpressionMethods::and())"] pub type And < Lhs , Rhs , ST = < Rhs as PreferredBoolSqlType > :: PreferredSqlType > = Grouped < super :: operators :: And < Lhs , AsExprOf < Rhs , ST > > > ;
};
}
