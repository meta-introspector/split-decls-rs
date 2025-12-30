// Generated macro for LtEq (type)
macro_rules! Depcrate_expression_helper_typesLtEq {
() => {
// Module: crate::expression::helper_types
// Provides: {"LtEq"}
// Dependencies: {}
# [doc = " The return type of"] # [doc = " [`lhs.le(rhs)`](crate::expression_methods::ExpressionMethods::le())"] pub type LtEq < Lhs , Rhs > = Grouped < super :: operators :: LtEq < Lhs , AsExpr < Rhs , Lhs > > > ;
};
}
