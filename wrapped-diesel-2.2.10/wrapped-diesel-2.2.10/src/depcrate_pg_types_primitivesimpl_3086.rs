// Generated macro for impl_3086 (impl)
macro_rules! Depcrate_pg_types_primitivesimpl_3086 {
() => {
// Module: crate::pg::types::primitives
// Provides: {"impl_3086"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl Queryable < sql_types :: Binary , Pg > for * const [u8] { type Row = Self ; fn build (row : Self :: Row) -> deserialize :: Result < Self > { Ok (row) } }
};
}
