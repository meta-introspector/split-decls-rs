// Generated macro for Escape (type)
macro_rules! Depcrate_expression_helper_typesEscape {
() => {
// Module: crate::expression::helper_types
// Provides: {"Escape"}
// Dependencies: {}
# [doc = " The return type of"] # [doc = " [`lhs.escape('x')`](crate::expression_methods::EscapeExpressionMethods::escape())"] pub type Escape < Lhs > = Grouped < super :: operators :: Escape < < Lhs as crate :: expression_methods :: EscapeExpressionMethods > :: TextExpression , AsExprOf < String , sql_types :: VarChar > , > , > ;
};
}
