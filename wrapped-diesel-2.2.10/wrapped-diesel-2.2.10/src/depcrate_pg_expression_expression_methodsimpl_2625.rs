// Generated macro for impl_2625 (impl)
macro_rules! Depcrate_pg_expression_expression_methodsimpl_2625 {
() => {
// Module: crate::pg::expression::expression_methods
// Provides: {"impl_2625"}
// Dependencies: {}
impl < T , U > EscapeExpressionMethods for Grouped < ILike < T , U > > { type TextExpression = ILike < T , U > ; fn escape (self , character : char) -> dsl :: Escape < Self > { Grouped (crate :: expression :: operators :: Escape :: new (self . 0 , character . to_string () . into_sql :: < VarChar > () ,)) } }
};
}
