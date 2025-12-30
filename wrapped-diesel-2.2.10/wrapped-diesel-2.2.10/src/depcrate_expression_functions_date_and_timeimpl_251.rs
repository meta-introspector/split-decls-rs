// Generated macro for impl_251 (impl)
macro_rules! Depcrate_expression_functions_date_and_timeimpl_251 {
() => {
// Module: crate::expression::functions::date_and_time
// Provides: {"impl_251"}
// Dependencies: {}
impl AsExpression < Nullable < Timestamp > > for now { type Expression = Coerce < now , Nullable < Timestamp > > ; fn as_expression (self) -> Self :: Expression { Coerce :: new (self) } }
};
}
