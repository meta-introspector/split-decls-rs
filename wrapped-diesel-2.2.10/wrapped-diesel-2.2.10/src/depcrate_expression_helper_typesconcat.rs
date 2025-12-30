// Generated macro for Concat (type)
macro_rules! Depcrate_expression_helper_typesConcat {
() => {
// Module: crate::expression::helper_types
// Provides: {"Concat"}
// Dependencies: {}
# [doc = " The return type of"] # [doc = " [`lhs.concat(rhs)`](crate::expression_methods::TextExpressionMethods::concat())"] pub type Concat < Lhs , Rhs > = Grouped < super :: operators :: Concat < Lhs , AsExpr < Rhs , Lhs > > > ;
};
}
