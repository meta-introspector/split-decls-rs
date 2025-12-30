// Generated macro for impl_3103 (impl)
macro_rules! Depcrate_pg_types_rangesimpl_3103 {
() => {
// Module: crate::pg::types::ranges
// Provides: {"impl_3103"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl < ST : 'static , T > AsExpression < Nullable < Range < ST > > > for (Bound < T > , Bound < T >) { type Expression = SqlBound < Nullable < Range < ST > > , Self > ; fn as_expression (self) -> Self :: Expression { SqlBound :: new (self) } }
};
}
