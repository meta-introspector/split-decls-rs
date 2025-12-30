// Generated macro for impl_262 (impl)
macro_rules! Depcrate_expression_functions_date_and_timeimpl_262 {
() => {
// Module: crate::expression::functions::date_and_time
// Provides: {"impl_262"}
// Dependencies: {}
impl AsExpression < Nullable < Date > > for today { type Expression = Coerce < today , Nullable < Date > > ; fn as_expression (self) -> Self :: Expression { Coerce :: new (self) } }
};
}
