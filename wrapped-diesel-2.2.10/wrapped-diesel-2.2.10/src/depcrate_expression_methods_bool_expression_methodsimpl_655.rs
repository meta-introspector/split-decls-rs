// Generated macro for impl_655 (impl)
macro_rules! Depcrate_expression_methods_bool_expression_methodsimpl_655 {
() => {
// Module: crate::expression_methods::bool_expression_methods
// Provides: {"impl_655"}
// Dependencies: {}
# [doc = " This impl has to live in Diesel because otherwise it would conflict with the blanket impl above"] # [doc = " because \"diesel might add an implementation of Expression for bool\""] impl PreferredBoolSqlType for bool { type PreferredSqlType = sql_types :: Bool ; }
};
}
