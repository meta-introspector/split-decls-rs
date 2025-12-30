// Generated macro for impl_253 (impl)
macro_rules! Depcrate_expression_functions_date_and_timeimpl_253 {
() => {
// Module: crate::expression::functions::date_and_time
// Provides: {"impl_253"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl AsExpression < Nullable < Timestamptz > > for now { type Expression = Coerce < now , Nullable < Timestamptz > > ; fn as_expression (self) -> Self :: Expression { Coerce :: new (self) } }
};
}
