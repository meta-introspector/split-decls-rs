// Generated macro for NotLike (type)
macro_rules! Depcrate_expression_helper_typesNotLike {
() => {
// Module: crate::expression::helper_types
// Provides: {"NotLike"}
// Dependencies: {}
# [doc = " The return type of"] # [doc = " [`lhs.not_like(rhs)`](crate::expression_methods::TextExpressionMethods::not_like())"] pub type NotLike < Lhs , Rhs > = Grouped < super :: operators :: NotLike < Lhs , AsExprOf < Rhs , SqlTypeOf < Lhs > > > > ;
};
}
