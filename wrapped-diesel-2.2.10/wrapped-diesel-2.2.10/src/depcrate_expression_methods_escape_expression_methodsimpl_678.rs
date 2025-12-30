// Generated macro for impl_678 (impl)
macro_rules! Depcrate_expression_methods_escape_expression_methodsimpl_678 {
() => {
// Module: crate::expression_methods::escape_expression_methods
// Provides: {"impl_678"}
// Dependencies: {}
impl < T , U > EscapeExpressionMethods for Grouped < Like < T , U > > { type TextExpression = Like < T , U > ; fn escape (self , character : char) -> dsl :: Escape < Self > { Grouped (Escape :: new (self . 0 , character . to_string () . into_sql :: < VarChar > () ,)) } }
};
}
