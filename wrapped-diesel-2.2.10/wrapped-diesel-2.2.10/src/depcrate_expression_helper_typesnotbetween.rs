// Generated macro for NotBetween (type)
macro_rules! Depcrate_expression_helper_typesNotBetween {
() => {
// Module: crate::expression::helper_types
// Provides: {"NotBetween"}
// Dependencies: {}
# [doc = " The return type of"] # [doc = " [`lhs.not_between(lower, upper)`](crate::expression_methods::ExpressionMethods::not_between())"] pub type NotBetween < Lhs , Lower , Upper > = Grouped < super :: operators :: NotBetween < Lhs , super :: operators :: And < AsExpr < Lower , Lhs > , AsExpr < Upper , Lhs > > , > , > ;
};
}
