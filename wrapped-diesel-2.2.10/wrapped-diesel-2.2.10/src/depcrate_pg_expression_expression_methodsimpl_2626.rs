// Generated macro for impl_2626 (impl)
macro_rules! Depcrate_pg_expression_expression_methodsimpl_2626 {
() => {
// Module: crate::pg::expression::expression_methods
// Provides: {"impl_2626"}
// Dependencies: {}
impl < T , U > EscapeExpressionMethods for Grouped < NotILike < T , U > > { type TextExpression = NotILike < T , U > ; fn escape (self , character : char) -> dsl :: Escape < Self > { Grouped (crate :: expression :: operators :: Escape :: new (self . 0 , character . to_string () . into_sql :: < VarChar > () ,)) } }
};
}
