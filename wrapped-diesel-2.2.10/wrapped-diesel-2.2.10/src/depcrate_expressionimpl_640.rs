// Generated macro for impl_640 (impl)
macro_rules! Depcrate_expressionimpl_640 {
() => {
// Module: crate::expression
// Provides: {"impl_640"}
// Dependencies: {}
impl < QS , T , DB , GB , IsAggregate > BoxableExpression < QS , DB , GB , IsAggregate > for T where DB : Backend , T : Expression , T : SelectableExpression < QS > , T : ValidGrouping < GB > , T : QueryFragment < DB > , T : Send , T :: IsAggregate : MixedAggregates < IsAggregate , Output = IsAggregate > , { }
};
}
