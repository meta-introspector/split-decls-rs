// Generated macro for impl_3074 (impl)
macro_rules! Depcrate_pg_types_primitivesimpl_3074 {
() => {
// Module: crate::pg::types::primitives
// Provides: {"impl_3074"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl FromSql < sql_types :: Bool , Pg > for bool { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Self > { Ok (bytes . as_bytes () [0] != 0) } }
};
}
