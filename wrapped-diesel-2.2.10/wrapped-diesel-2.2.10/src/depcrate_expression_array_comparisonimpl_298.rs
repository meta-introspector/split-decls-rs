// Generated macro for impl_298 (impl)
macro_rules! Depcrate_expression_array_comparisonimpl_298 {
() => {
// Module: crate::expression::array_comparison
// Provides: {"impl_298"}
// Dependencies: {}
impl < ST , F , S , D , W , O , LOf , G , H , LC > AsInExpression < ST > for SelectStatement < F , S , D , W , O , LOf , G , H , LC > where ST : SqlType + TypedExpressionType , Subselect < Self , ST > : Expression < SqlType = ST > , Self : SelectQuery < SqlType = ST > , { type InExpression = Subselect < Self , ST > ; fn as_in_expression (self) -> Self :: InExpression { Subselect :: new (self) } }
};
}
