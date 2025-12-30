// Generated macro for impl_465 (impl)
macro_rules! Depcrate_expression_nullableimpl_465 {
() => {
// Module: crate::expression::nullable
// Provides: {"impl_465"}
// Dependencies: {}
impl < T > Expression for Nullable < T > where T : Expression , T :: SqlType : IntoNullable , < T :: SqlType as IntoNullable > :: Nullable : TypedExpressionType , { type SqlType = < T :: SqlType as IntoNullable > :: Nullable ; }
};
}
