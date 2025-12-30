// Generated macro for impl_3102 (impl)
macro_rules! Depcrate_pg_types_rangesimpl_3102 {
() => {
// Module: crate::pg::types::ranges
// Provides: {"impl_3102"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl < ST : 'static , T > AsExpression < Range < ST > > for & (Bound < T > , Bound < T >) { type Expression = SqlBound < Range < ST > , Self > ; fn as_expression (self) -> Self :: Expression { SqlBound :: new (self) } }
};
}
