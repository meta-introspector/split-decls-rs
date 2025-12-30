// Generated macro for impl_2628 (impl)
macro_rules! Depcrate_pg_expression_expression_methodsimpl_2628 {
() => {
// Module: crate::pg::expression::expression_methods
// Provides: {"impl_2628"}
// Dependencies: {}
impl < T , U > EscapeExpressionMethods for Grouped < NotSimilarTo < T , U > > { type TextExpression = NotSimilarTo < T , U > ; fn escape (self , character : char) -> dsl :: Escape < Self > { Grouped (crate :: expression :: operators :: Escape :: new (self . 0 , character . to_string () . into_sql :: < VarChar > () ,)) } }
};
}
