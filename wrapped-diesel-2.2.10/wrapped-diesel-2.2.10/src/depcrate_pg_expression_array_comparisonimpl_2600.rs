// Generated macro for impl_2600 (impl)
macro_rules! Depcrate_pg_expression_array_comparisonimpl_2600 {
() => {
// Module: crate::pg::expression::array_comparison
// Provides: {"impl_2600"}
// Dependencies: {}
impl < ST , T > AsArrayExpression < ST > for T where ST : 'static , T : AsExpression < Array < ST > > , { type Expression = < T as AsExpression < Array < ST > > > :: Expression ; fn as_expression (self) -> Self :: Expression { < T as AsExpression < Array < ST > > > :: as_expression (self) } }
};
}
