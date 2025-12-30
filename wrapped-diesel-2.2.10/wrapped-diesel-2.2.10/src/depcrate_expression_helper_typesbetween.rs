// Generated macro for Between (type)
macro_rules! Depcrate_expression_helper_typesBetween {
() => {
// Module: crate::expression::helper_types
// Provides: {"Between"}
// Dependencies: {}
# [doc = " The return type of"] # [doc = " [`lhs.between(lower, upper)`](crate::expression_methods::ExpressionMethods::between())"] pub type Between < Lhs , Lower , Upper > = Grouped < super :: operators :: Between < Lhs , super :: operators :: And < AsExpr < Lower , Lhs > , AsExpr < Upper , Lhs > > > , > ;
};
}
