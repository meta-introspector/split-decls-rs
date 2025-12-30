// Generated macro for impl_469 (impl)
macro_rules! Depcrate_expression_nullableimpl_469 {
() => {
// Module: crate::expression::nullable
// Provides: {"impl_469"}
// Dependencies: {}
impl < T , QS > SelectableExpression < QS > for Nullable < T > where Self : AppearsOnTable < QS > , QS : ToInnerJoin , T : SelectableExpression < QS :: InnerJoin > , { }
};
}
