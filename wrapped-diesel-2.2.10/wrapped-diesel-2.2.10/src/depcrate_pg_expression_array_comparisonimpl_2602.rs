// Generated macro for impl_2602 (impl)
macro_rules! Depcrate_pg_expression_array_comparisonimpl_2602 {
() => {
// Module: crate::pg::expression::array_comparison
// Provides: {"impl_2602"}
// Dependencies: {}
impl < ST , QS , DB , GB > AsArrayExpression < ST > for BoxedSelectStatement < '_ , ST , QS , DB , GB > where ST : 'static , Self : SelectQuery < SqlType = ST > , { type Expression = Subselect < Self , Array < ST > > ; fn as_expression (self) -> Self :: Expression { Subselect :: new (self) } }
};
}
