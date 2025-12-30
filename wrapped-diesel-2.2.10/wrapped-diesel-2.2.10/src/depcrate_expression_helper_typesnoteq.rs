// Generated macro for NotEq (type)
macro_rules! Depcrate_expression_helper_typesNotEq {
() => {
// Module: crate::expression::helper_types
// Provides: {"NotEq"}
// Dependencies: {}
# [doc = " The return type of"] # [doc = " [`lhs.ne(rhs)`](crate::expression_methods::ExpressionMethods::ne())"] pub type NotEq < Lhs , Rhs > = Grouped < super :: operators :: NotEq < Lhs , AsExpr < Rhs , Lhs > > > ;
};
}
