// Generated macro for impl_654 (impl)
macro_rules! Depcrate_expression_methods_bool_expression_methodsimpl_654 {
() => {
// Module: crate::expression_methods::bool_expression_methods
// Provides: {"impl_654"}
// Dependencies: {}
impl < E : Expression > PreferredBoolSqlType for E where E :: SqlType : BoolOrNullableBool , { type PreferredSqlType = < E as Expression > :: SqlType ; }
};
}
