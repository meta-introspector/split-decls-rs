// Generated macro for Gt (type)
macro_rules! Depcrate_expression_helper_typesGt {
() => {
// Module: crate::expression::helper_types
// Provides: {"Gt"}
// Dependencies: {}
# [doc = " The return type of"] # [doc = " [`lhs.gt(rhs)`](crate::expression_methods::ExpressionMethods::gt())"] pub type Gt < Lhs , Rhs > = Grouped < super :: operators :: Gt < Lhs , AsExpr < Rhs , Lhs > > > ;
};
}
