// Generated macro for impl_2601 (impl)
macro_rules! Depcrate_pg_expression_array_comparisonimpl_2601 {
() => {
// Module: crate::pg::expression::array_comparison
// Provides: {"impl_2601"}
// Dependencies: {}
impl < ST , F , S , D , W , O , LOf , G , H , LC > AsArrayExpression < ST > for SelectStatement < F , S , D , W , O , LOf , G , H , LC > where ST : 'static , Self : SelectQuery < SqlType = ST > , { type Expression = Subselect < Self , Array < ST > > ; fn as_expression (self) -> Self :: Expression { Subselect :: new (self) } }
};
}
