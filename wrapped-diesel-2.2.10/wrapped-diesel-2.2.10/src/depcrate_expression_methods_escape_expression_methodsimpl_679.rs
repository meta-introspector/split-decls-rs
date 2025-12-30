// Generated macro for impl_679 (impl)
macro_rules! Depcrate_expression_methods_escape_expression_methodsimpl_679 {
() => {
// Module: crate::expression_methods::escape_expression_methods
// Provides: {"impl_679"}
// Dependencies: {}
impl < T , U > EscapeExpressionMethods for Grouped < NotLike < T , U > > { type TextExpression = NotLike < T , U > ; fn escape (self , character : char) -> dsl :: Escape < Self > { Grouped (Escape :: new (self . 0 , character . to_string () . into_sql :: < VarChar > () ,)) } }
};
}
