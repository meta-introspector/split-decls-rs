// Generated macro for impl_252 (impl)
macro_rules! Depcrate_expression_functions_date_and_timeimpl_252 {
() => {
// Module: crate::expression::functions::date_and_time
// Provides: {"impl_252"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl AsExpression < Timestamptz > for now { type Expression = Coerce < now , Timestamptz > ; fn as_expression (self) -> Self :: Expression { Coerce :: new (self) } }
};
}
