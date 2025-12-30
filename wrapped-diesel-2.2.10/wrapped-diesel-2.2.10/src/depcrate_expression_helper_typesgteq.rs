// Generated macro for GtEq (type)
macro_rules! Depcrate_expression_helper_typesGtEq {
() => {
// Module: crate::expression::helper_types
// Provides: {"GtEq"}
// Dependencies: {}
# [doc = " The return type of"] # [doc = " [`lhs.ge(rhs)`](crate::expression_methods::ExpressionMethods::ge())"] pub type GtEq < Lhs , Rhs > = Grouped < super :: operators :: GtEq < Lhs , AsExpr < Rhs , Lhs > > > ;
};
}
