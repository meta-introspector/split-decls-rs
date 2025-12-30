// Generated macro for impl_299 (impl)
macro_rules! Depcrate_expression_array_comparisonimpl_299 {
() => {
// Module: crate::expression::array_comparison
// Provides: {"impl_299"}
// Dependencies: {}
impl < 'a , ST , QS , DB , GB > AsInExpression < ST > for BoxedSelectStatement < 'a , ST , QS , DB , GB > where ST : SqlType + TypedExpressionType , Subselect < BoxedSelectStatement < 'a , ST , QS , DB , GB > , ST > : Expression < SqlType = ST > , { type InExpression = Subselect < Self , ST > ; fn as_in_expression (self) -> Self :: InExpression { Subselect :: new (self) } }
};
}
