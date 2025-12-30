// Generated macro for impl_611 (impl)
macro_rules! Depcrate_expressionimpl_611 {
() => {
// Module: crate::expression
// Provides: {"impl_611"}
// Dependencies: {}
impl < T , ST > AsExpression < ST > for T where T : Expression < SqlType = ST > , ST : SqlType + TypedExpressionType , { type Expression = T ; fn as_expression (self) -> T { self } }
};
}
