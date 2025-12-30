// Generated macro for impl_3101 (impl)
macro_rules! Depcrate_pg_types_rangesimpl_3101 {
() => {
// Module: crate::pg::types::ranges
// Provides: {"impl_3101"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl < ST : 'static , T > AsExpression < Range < ST > > for (Bound < T > , Bound < T >) { type Expression = SqlBound < Range < ST > , Self > ; fn as_expression (self) -> Self :: Expression { SqlBound :: new (self) } }
};
}
