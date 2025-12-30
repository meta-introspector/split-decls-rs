// Generated macro for impl_545 (impl)
macro_rules! Depcrate_expression_select_byimpl_545 {
() => {
// Module: crate::expression::select_by
// Provides: {"impl_545"}
// Dependencies: {}
impl < T , E , DB > Expression for SelectBy < T , DB > where DB : Backend , T : Selectable < DB , SelectExpression = E > , E : QueryId + Expression , { type SqlType = SelectBy < T , DB > ; }
};
}
