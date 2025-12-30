// Generated macro for impl_348 (impl)
macro_rules! Depcrate_expression_coerceimpl_348 {
() => {
// Module: crate::expression::coerce
// Provides: {"impl_348"}
// Dependencies: {}
impl < T , ST > Expression for Coerce < T , ST > where T : Expression , ST : SqlType + TypedExpressionType , { type SqlType = ST ; }
};
}
