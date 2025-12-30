// Generated macro for impl_254 (impl)
macro_rules! Depcrate_expression_functions_date_and_timeimpl_254 {
() => {
// Module: crate::expression::functions::date_and_time
// Provides: {"impl_254"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl AsExpression < TimestamptzSqlite > for now { type Expression = Coerce < now , TimestamptzSqlite > ; fn as_expression (self) -> Self :: Expression { Coerce :: new (self) } }
};
}
