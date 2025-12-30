// Generated macro for impl_300 (impl)
macro_rules! Depcrate_expression_array_comparisonimpl_300 {
() => {
// Module: crate::expression::array_comparison
// Provides: {"impl_300"}
// Dependencies: {}
impl < ST , Combinator , Rule , Source , Rhs > AsInExpression < ST > for CombinationClause < Combinator , Rule , Source , Rhs > where ST : SqlType + TypedExpressionType , Self : SelectQuery < SqlType = ST > , Subselect < Self , ST > : Expression < SqlType = ST > , { type InExpression = Subselect < Self , ST > ; fn as_in_expression (self) -> Self :: InExpression { Subselect :: new (self) } }
};
}
