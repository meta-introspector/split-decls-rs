// Generated macro for Eq (type)
macro_rules! Depcrate_expression_helper_typesEq {
() => {
// Module: crate::expression::helper_types
// Provides: {"Eq"}
// Dependencies: {}
# [doc = " The return type of"] # [doc = " [`lhs.eq(rhs)`](crate::expression_methods::ExpressionMethods::eq())"] pub type Eq < Lhs , Rhs > = Grouped < super :: operators :: Eq < Lhs , AsExpr < Rhs , Lhs > > > ;
};
}
