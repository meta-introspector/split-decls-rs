// Generated macro for Like (type)
macro_rules! Depcrate_expression_helper_typesLike {
() => {
// Module: crate::expression::helper_types
// Provides: {"Like"}
// Dependencies: {}
# [doc = " The return type of"] # [doc = " [`lhs.like(rhs)`](crate::expression_methods::TextExpressionMethods::like())"] pub type Like < Lhs , Rhs > = Grouped < super :: operators :: Like < Lhs , AsExprOf < Rhs , SqlTypeOf < Lhs > > > > ;
};
}
