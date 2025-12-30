// Generated macro for impl_255 (impl)
macro_rules! Depcrate_expression_functions_date_and_timeimpl_255 {
() => {
// Module: crate::expression::functions::date_and_time
// Provides: {"impl_255"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl AsExpression < Nullable < TimestamptzSqlite > > for now { type Expression = Coerce < now , Nullable < TimestamptzSqlite > > ; fn as_expression (self) -> Self :: Expression { Coerce :: new (self) } }
};
}
