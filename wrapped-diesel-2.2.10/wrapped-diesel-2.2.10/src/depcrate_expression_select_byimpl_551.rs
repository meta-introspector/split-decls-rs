// Generated macro for impl_551 (impl)
macro_rules! Depcrate_expression_select_byimpl_551 {
() => {
// Module: crate::expression::select_by
// Provides: {"impl_551"}
// Dependencies: {}
impl < T , QS , DB > AppearsOnTable < QS > for SelectBy < T , DB > where DB : Backend , T : Selectable < DB > , T :: SelectExpression : AppearsOnTable < QS > , Self : Expression , { }
};
}
