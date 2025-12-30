// Generated macro for impl_588 (impl)
macro_rules! Depcrate_expression_subselectimpl_588 {
() => {
// Module: crate::expression::subselect
// Provides: {"impl_588"}
// Dependencies: {}
impl < T : SelectQuery , ST > Expression for Subselect < T , ST > where ST : SqlType + TypedExpressionType , { type SqlType = ST ; }
};
}
