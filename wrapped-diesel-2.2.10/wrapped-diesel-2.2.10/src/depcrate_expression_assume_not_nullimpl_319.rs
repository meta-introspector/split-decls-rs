// Generated macro for impl_319 (impl)
macro_rules! Depcrate_expression_assume_not_nullimpl_319 {
() => {
// Module: crate::expression::assume_not_null
// Provides: {"impl_319"}
// Dependencies: {}
impl < T > Expression for AssumeNotNull < T > where T : Expression , T :: SqlType : IntoNotNullable , < T :: SqlType as IntoNotNullable > :: NotNullable : TypedExpressionType , { type SqlType = < T :: SqlType as IntoNotNullable > :: NotNullable ; }
};
}
