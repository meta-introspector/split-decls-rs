// Generated macro for AtTimeZone (type)
macro_rules! Depcrate_pg_expression_helper_typesAtTimeZone {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"AtTimeZone"}
// Dependencies: {}
# [doc = " The return type of [`expr.at_time_zone(tz)`](super::expression_methods::PgTimestampExpressionMethods::at_time_zone)"] # [cfg (feature = "postgres_backend")] pub type AtTimeZone < Lhs , Rhs > = Grouped < super :: date_and_time :: AtTimeZone < Lhs , AsExprOf < Rhs , VarChar > > > ;
};
}
