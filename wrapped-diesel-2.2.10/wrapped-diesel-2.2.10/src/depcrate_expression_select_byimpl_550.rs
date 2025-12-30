// Generated macro for impl_550 (impl)
macro_rules! Depcrate_expression_select_byimpl_550 {
() => {
// Module: crate::expression::select_by
// Provides: {"impl_550"}
// Dependencies: {}
impl < T , QS , DB > SelectableExpression < QS > for SelectBy < T , DB > where DB : Backend , T : Selectable < DB > , T :: SelectExpression : SelectableExpression < QS > , Self : AppearsOnTable < QS > , { }
};
}
